import { commands, hooks } from "@commands";
import { TranslationContext, TranslationMap } from "@components/common/TranslationContext";
import { useTheme } from "@hooks";
import { getCurrent } from "@tauri-apps/api/window";
import { SocketMessageType, Theme } from "@types";
import { useEffect, useState } from "react";
import LogHeader from "@components/logs/LogHeader";
import LogList from "@components/logs/LogList";
import CenteredSpinner from "@components/common/CenteredSpinner";

export type LogFilter = keyof typeof SocketMessageType | "Any";

const thisWindow = getCurrent();

const port = parseInt(thisWindow.label.split("-")[1]);

let unlisten: () => void = () => null;

unlisten = await thisWindow.onCloseRequested((e) => {
    e.preventDefault();
    commands
        .stopLogging({ port })
        .catch(console.warn)
        .finally(() => {
            unlisten();
            thisWindow.close();
        });
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
                    "$port$",
                    port.toString()
                )
            )
            .catch(console.warn);
    }, [guiConfig?.language]);

    const logsLen = hooks.getLogsLength(`GAME-LOG-${port}`, { port })[1];

    if (guiConfig === null) {
        return <CenteredSpinner />;
    }

    return (
        <TranslationContext.Provider value={guiConfig!.language}>
            <main className="logs container">
                <LogHeader
                    autoScroll={autoScroll}
                    setAutoScroll={setAutoScroll}
                    activeFilter={activeFilter}
                    setActiveFilter={setActiveFilter}
                />
                <LogList
                    autoScroll={autoScroll}
                    activeFilter={activeFilter}
                    logsLen={logsLen ?? 0}
                    port={port}
                />
            </main>
        </TranslationContext.Provider>
    );
};

export default App;
