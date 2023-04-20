import { commands, hooks } from "@commands";
import { TranslationContext, TranslationMap } from "@components/common/TranslationContext";
import { useTheme, useTranslation } from "@hooks";
import { GameMessage, SocketMessageType, Theme } from "@types";
import { useCallback, useEffect, useState } from "react";
import LogHeader from "@components/logs/LogHeader";
import LogList from "@components/logs/LogList";
import CenteredSpinner from "@components/common/CenteredSpinner";
import { listen } from "@tauri-apps/api/event";
import { getCurrent } from "@tauri-apps/api/window";
import { dialog } from "@tauri-apps/api";

export type LogFilter = keyof typeof SocketMessageType | "Any";

const thisWindow = getCurrent();

const getFilterToPass = (activeFilter: LogFilter) => {
    if (activeFilter === "Any") {
        return undefined;
    } else {
        return Object.keys(SocketMessageType).indexOf(activeFilter);
    }
};

const App = ({ port }: { port: number }) => {
    const [activeFilter, setActiveFilter] = useState<LogFilter>("Any");
    const [activeSearch, setActiveSearch] = useState<string>("");
    const [autoScroll, setAutoScroll] = useState(true);
    const [logLines, setLogLines] = useState<[number, number][]>([]);

    const fatalErrorLabel = useTranslation("FATAL_ERROR");

    const fetchLogLines = useCallback(() => {
        commands
            .getLogLines({ port, filterType: getFilterToPass(activeFilter), search: activeSearch })
            .then(setLogLines)
            .catch(() => null);
    }, [activeFilter, activeSearch]);

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
    }, [guiConfig?.language, port]);

    const onClear = useCallback(() => {
        commands.clearLogs({ port }).catch(console.warn);
        setLogLines([]);
    }, []);

    useEffect(() => {
        let cancel = false;
        listen("LOG-UPDATE", (e) => {
            if (cancel || (e.payload as number) !== port) return;
            fetchLogLines();
        }).catch(console.warn);
        listen("LOG-FATAL", (e) => {
            const msg = e.payload as GameMessage;
            if (cancel || msg.port !== port) return;
            dialog.message(`[${msg.message.senderName ?? "Unknown"}]: ${msg.message.message}`, {
                type: "error",
                title: fatalErrorLabel
            });
        });
        return () => {
            cancel = true;
        };
    }, []);

    useEffect(() => {
        fetchLogLines();
    }, [activeFilter, activeSearch]);

    if (guiConfig === null || logLines === null) {
        return <CenteredSpinner />;
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
                        activeSearch={activeSearch}
                        setActiveSearch={setActiveSearch}
                        onClear={onClear}
                    />
                    <LogList
                        port={port}
                        autoScroll={autoScroll}
                        activeFilter={activeFilter}
                        search={activeSearch}
                        logLines={logLines ?? []}
                    />
                </main>
            </TranslationContext.Provider>
        );
    }
};

export default App;
