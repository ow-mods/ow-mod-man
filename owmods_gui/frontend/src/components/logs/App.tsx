import { commands, hooks } from "@commands";
import { TranslationMap } from "@components/TranslationContext";
import { useTheme } from "@hooks";
import { getCurrent } from "@tauri-apps/api/window";
import { SocketMessage, Theme } from "@types";
import { useCallback, useEffect, useRef } from "react";
import LogLine from "./LogLine";
import AutoSizer from "react-virtualized-auto-sizer";
import { ListOnItemsRenderedProps, VariableSizeList } from "react-window";

// Memoize nearbly log data so that way we dont read/write a bunch
const MEMOIZATION_RANGE = 30;

const thisWindow = getCurrent();

const port = parseInt(thisWindow.label.split("-")[1]);

let unlisten: () => void = () => null;

unlisten = await thisWindow.onCloseRequested((e) => {
    e.preventDefault();
    commands
        .stopLogging({ port })
        .then(() => {
            unlisten();
            thisWindow.close();
        })
        .catch(console.warn);
});

const App = () => {
    const listRef = useRef<VariableSizeList>(null);
    const guiConfig = hooks.getGuiConfig("GUI_CONFIG_RELOAD")[1];
    useTheme(guiConfig?.theme ?? Theme.White, guiConfig?.rainbow ?? false);

    const logSizes = useRef<Record<number, number>>({});
    const logMemory = useRef<Record<number, SocketMessage>>({});

    const reportSize = useCallback((i: number, size: number) => {
        logSizes.current[i] = size;
        listRef.current?.resetAfterIndex(i);
    }, []);
    const getSize = useCallback((i: number) => logSizes.current[i] ?? 40, []);

    const remember = useCallback((i: number, msg: SocketMessage) => {
        logMemory.current[i] = msg;
    }, []);
    const getCached = useCallback((i: number) => {
        return logMemory.current[i];
    }, []);

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

    useEffect(() => {
        setTimeout(() => {
            listRef.current?.scrollToItem(logsLen ?? 0, "end");
        }, 100);
    }, [logsLen, logSizes.current]);

    const onScroll = useCallback((data: ListOnItemsRenderedProps) => {
        const len = Object.keys(logMemory).length;
        if (len > MEMOIZATION_RANGE) {
            const lower = Math.max(data.overscanStartIndex - MEMOIZATION_RANGE, 0);
            const upper = Math.min(data.overscanStopIndex + MEMOIZATION_RANGE, len);
            for (const k of Object.keys(logMemory.current) as unknown as number[]) {
                if (k < lower || k > upper) {
                    delete logMemory.current[k];
                }
            }
        }
    }, []);

    return (
        <main className="logs container">
            <AutoSizer>
                {(size) => (
                    <VariableSizeList
                        ref={listRef}
                        overscanCount={10}
                        itemCount={logsLen ?? 0}
                        itemSize={getSize}
                        onItemsRendered={onScroll}
                        {...size}
                    >
                        {({ index, style }) => (
                            <LogLine
                                reportSize={reportSize}
                                port={port}
                                line={index}
                                key={index}
                                style={style}
                                getCached={getCached}
                                remember={remember}
                            />
                        )}
                    </VariableSizeList>
                )}
            </AutoSizer>
        </main>
    );
};

export default App;
