import { commands, hooks } from "@commands";
import BaseApp from "@components/common/BaseApp";
import { TranslationMap } from "@components/common/TranslationContext";
import { useGetTranslation } from "@hooks";
import { dialog } from "@tauri-apps/api";
import { getCurrent } from "@tauri-apps/api/window";
import { SocketMessageType } from "@types";
import { useState, useCallback, useEffect } from "react";
import LogHeader from "./LogHeader";
import { Container, useTheme } from "@mui/material";
import LogTable from "./LogTable";
import { listen } from "@events";
import { simpleOnError } from "@components/common/StyledErrorBoundary";

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
    const [status, guiConfig] = hooks.getGuiConfig("guiConfigReload");

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
                .catch(simpleOnError);
        }
    }, [guiConfig?.language, port]);

    const onClear = useCallback(() => {
        commands.clearLogs({ port }).catch(simpleOnError);
        setLogLines([]);
    }, [port]);

    useEffect(() => {
        const unsubscribe = listen("logUpdate", (portPayload) => {
            if (portPayload !== port) return;
            fetchLogLines();
        });
        listen("logFatal", (msg) => {
            if (msg.port !== port) return;
            dialog.message(`[${msg.message.senderName ?? "Unknown"}]: ${msg.message.message}`, {
                type: "error",
                title: getTranslation("FATAL_ERROR")
            });
        });
        return unsubscribe;
    }, [fetchLogLines, getTranslation, port]);

    useEffect(() => {
        fetchLogLines();
    }, [activeFilter, activeSearch, fetchLogLines]);

    return (
        <BaseApp
            language={guiConfig?.language}
            theme={guiConfig?.theme}
            isLoading={status === "Loading" || logLines === null}
        >
            <Container
                disableGutters
                sx={{
                    display: "flex",
                    flexDirection: "column",
                    gap: theme.spacing(1),
                    flexGrow: 1,
                    maxHeight: "100%",
                    padding: theme.spacing(1)
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
