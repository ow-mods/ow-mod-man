import { commands, hooks } from "@commands";
import BaseApp from "@components/common/BaseApp";
import { TranslationMap } from "@components/common/TranslationContext";
import { useGetTranslation } from "@hooks";
import { dialog } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { getCurrent } from "@tauri-apps/api/window";
import { GameMessage, SocketMessageType } from "@types";
import { useState, useCallback, useEffect } from "react";
import LogHeader from "./LogHeader";
import { Container, useTheme } from "@mui/material";
import LogTable from "./LogTable";

export type LogFilter = keyof typeof SocketMessageType | "Any";
export type LogLines = [number, number][];

const thisWindow = getCurrent();

const getFilterToPass = (activeFilter: LogFilter) => {
    if (activeFilter === "Any") {
        return undefined;
    } else {
        return Object.keys(SocketMessageType).indexOf(activeFilter);
    }
};

const LogApp = ({ port }: { port: number }) => {
    const [status, guiConfig, err] = hooks.getGuiConfig("GUI_CONFIG_RELOAD");

    const [activeFilter, setActiveFilter] = useState<LogFilter>("Any");
    const [activeSearch, setActiveSearch] = useState<string>("");
    const [logLines, setLogLines] = useState<LogLines>([]);
    const getTranslation = useGetTranslation();
    const theme = useTheme();

    const fetchLogLines = useCallback(() => {
        commands
            .getLogLines({ port, filterType: getFilterToPass(activeFilter), search: activeSearch })
            .then(setLogLines)
            .catch(() => null);
    }, [activeFilter, activeSearch, port]);

    useEffect(() => {
        const logsTitleTranslation = TranslationMap[guiConfig?.language ?? "English"]["LOGS_TITLE"];
        if (logsTitleTranslation) {
            thisWindow
                .setTitle(logsTitleTranslation.replace("$port$", port.toString()))
                .catch(console.warn);
        }
    }, [guiConfig?.language, port]);

    const onClear = useCallback(() => {
        commands.clearLogs({ port }).catch(console.warn);
        setLogLines([]);
    }, [port]);

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
                title: getTranslation("FATAL_ERROR")
            });
        });
        return () => {
            cancel = true;
        };
    }, [fetchLogLines, getTranslation, port]);

    useEffect(() => {
        fetchLogLines();
    }, [activeFilter, activeSearch, fetchLogLines]);

    return (
        <BaseApp
            language={guiConfig?.language}
            isLoading={status === "Loading" || logLines === null}
            fatalError={err?.toString()}
        >
            <Container
                sx={{
                    display: "flex",
                    flexDirection: "column",
                    gap: theme.spacing(3),
                    flexGrow: 1,
                    maxHeight: "100%",
                    paddingY: theme.spacing(3)
                }}
            >
                <LogHeader
                    onClear={onClear}
                    logsLen={logLines.length}
                    activeSearch={activeSearch}
                    setActiveSearch={setActiveSearch}
                    activeFilter={activeFilter}
                    setActiveFilter={setActiveFilter}
                />
                <LogTable logLines={logLines} port={port} />
            </Container>
        </BaseApp>
    );
};

export default LogApp;