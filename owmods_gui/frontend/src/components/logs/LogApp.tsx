import { commands, hooks } from "@commands";
import BaseApp from "@components/common/BaseApp";
import { TranslationMap } from "@components/common/TranslationContext";
import { useGetTranslation } from "@hooks";
import * as dialog from "@tauri-apps/plugin-dialog";
import { getCurrent } from "@tauri-apps/api/window";
import { SocketMessageType } from "@types";
import { useState, useCallback, useEffect, memo, useRef } from "react";
import LogHeader from "./LogHeader";
import { Container, useTheme } from "@mui/material";
import LogTable from "./LogTable";
import { listen } from "@events";
import { simpleOnError } from "../../errorHandling";

export type LogFilter = keyof typeof SocketMessageType | "Any";
export type LogLines = number[];

const thisWindow = getCurrent();

const getFilterToPass = (activeFilter: LogFilter) => {
    if (activeFilter === "Any") {
        return undefined;
    } else {
        return Object.keys(SocketMessageType).indexOf(activeFilter);
    }
};

const InnerLogApp = memo(function InnerLogApp({ port }: { port: number }) {
    const [activeFilter, setActiveFilter] = useState<LogFilter>("Any");
    const [activeSearch, setActiveSearch] = useState<string>("");
    const [logsBehind, setLogsBehind] = useState<boolean>(false);
    const [logLines, setLogLines] = useState<LogLines>([]);
    const [logTotal, setLogTotal] = useState<number>(0);
    const forceLogUpdateTimeout = useRef<ReturnType<typeof setTimeout> | null>(null);
    const getTranslation = useGetTranslation();
    const theme = useTheme();

    const fetchLogLines = useCallback(() => {
        commands
            .getLogLines({ port, filterType: getFilterToPass(activeFilter), search: activeSearch })
            .then(([lines, total]) => {
                setLogLines(lines);
                setLogTotal(total);
            })
            .catch(() => null);
    }, [activeFilter, activeSearch, port]);

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

    // In the event that logs are behind, enqueue a log update in 1 second if no other log update has been enqueued
    useEffect(() => {
        const unsubscribe = listen("logsBehind", (payload) => {
            if (payload.port !== port) return;
            setLogsBehind(payload.behind);
            if (forceLogUpdateTimeout.current !== null) {
                clearTimeout(forceLogUpdateTimeout.current);
                forceLogUpdateTimeout.current = null;
            }
            if (!payload.behind) return;
            forceLogUpdateTimeout.current = setTimeout(() => {
                if (forceLogUpdateTimeout.current !== null) {
                    commands
                        .forceLogUpdate({ port })
                        .catch(simpleOnError)
                        .then(() => {
                            forceLogUpdateTimeout.current = null;
                            setLogsBehind(false);
                        });
                }
            }, 1000);
        });
        return unsubscribe;
    }, [port]);

    useEffect(() => {
        fetchLogLines();
    }, [activeFilter, activeSearch, fetchLogLines]);

    return (
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
                logsLen={logTotal}
                activeSearch={activeSearch}
                setActiveSearch={setActiveSearch}
                activeFilter={activeFilter}
                isBehind={logsBehind}
                setActiveFilter={setActiveFilter}
            />
            <LogTable logLines={logLines} port={port} />
        </Container>
    );
});

const LogApp = ({ port }: { port: number }) => {
    const [status, guiConfig] = hooks.getGuiConfig("guiConfigReload");

    useEffect(() => {
        const logsTitleTranslation = TranslationMap[guiConfig?.language ?? "English"]["LOGS_TITLE"];
        if (logsTitleTranslation) {
            thisWindow
                .setTitle(logsTitleTranslation.replace("$port$", port.toString()))
                .catch(simpleOnError);
        }
    }, [guiConfig?.language, port]);

    return (
        <BaseApp
            language={guiConfig?.language}
            theme={guiConfig?.theme}
            usesRainbow={guiConfig?.rainbow}
            isLoading={status === "Loading" && guiConfig === null}
        >
            <InnerLogApp port={port} />
        </BaseApp>
    );
};

export default LogApp;
