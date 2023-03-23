import { commands, hooks } from "@commands";
import { TranslationContext, TranslationMap } from "@components/common/TranslationContext";
import { useTheme } from "@hooks";
import { getCurrent } from "@tauri-apps/api/window";
import { SocketMessageType, Theme } from "@types";
import { useCallback, useEffect, useMemo, useState } from "react";
import LogHeader from "@components/logs/LogHeader";
import LogList from "@components/logs/LogList";
import CenteredSpinner from "@components/common/CenteredSpinner";
import { startConsoleLogListen } from "../../logging";
import { listen } from "@tauri-apps/api/event";

export type LogFilter = keyof typeof SocketMessageType | "Any";

startConsoleLogListen();

const thisWindow = getCurrent();

let unlisten: () => void = () => null;

thisWindow
    .onCloseRequested((e) => {
        e.preventDefault();
        commands
            .stopLogging()
            .catch(console.warn)
            .finally(() => {
                unlisten();
                thisWindow.close();
            });
    })
    .then((u) => {
        unlisten = u;
    });

const ports: number[] = [];

listen("GAME-START", (e) => {
    const port = e.payload as number;
    if (!ports.includes(port)) ports.push(port);
});

const App = () => {
    const [activeFilter, setActiveFilter] = useState<LogFilter>("Any");
    const [autoScroll, setAutoScroll] = useState(true);

    const guiConfig = hooks.getGuiConfig("GUI_CONFIG_RELOAD")[1];

    useTheme(guiConfig?.theme ?? Theme.White, guiConfig?.rainbow ?? false);

    useEffect(() => {
        thisWindow
            .setTitle(
                TranslationMap[guiConfig?.language ?? "English"]["LOGS_TITLE"].replace(
                    "$ports$",
                    ports.join(", ")
                )
            )
            .catch(console.warn);
    }, [guiConfig?.language, ports]);

    const filterToPass = useMemo(() => {
        if (activeFilter === "Any") {
            return undefined;
        } else {
            return Object.keys(SocketMessageType).indexOf(activeFilter);
        }
    }, [activeFilter]);

    const onClear = useCallback(() => {
        commands.clearLogs().catch(console.warn);
    }, []);

    const [status, logLines, err] = hooks.getLogLines(`LOG-UPDATE`, { filterType: filterToPass });

    console.debug(logLines);

    if (guiConfig === null || (status === "Loading" && logLines === null)) {
        return <CenteredSpinner />;
    } else if (status === "Error") {
        return <div className="center">{err!.toString()}</div>;
    } else {
        return (
            <TranslationContext.Provider value={guiConfig!.language}>
                <main className="logs container">
                    <LogHeader
                        logsLen={logLines?.length ?? 0}
                        autoScroll={autoScroll}
                        setAutoScroll={setAutoScroll}
                        activeFilter={activeFilter}
                        setActiveFilter={setActiveFilter}
                        onClear={onClear}
                    />
                    <LogList
                        autoScroll={autoScroll}
                        activeFilter={activeFilter}
                        logLines={logLines ?? []}
                    />
                </main>
            </TranslationContext.Provider>
        );
    }
};

export default App;
