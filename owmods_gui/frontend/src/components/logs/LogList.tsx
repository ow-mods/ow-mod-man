import LogLine from "./LogLine";
import AutoSizer from "react-virtualized-auto-sizer";
import { memo, useCallback, useEffect, useRef } from "react";
import { ListOnItemsRenderedProps, VariableSizeList } from "react-window";
import { LogFilter } from "./LogApp";
import { SocketMessage } from "@types";

// Memoize nearby log data so that way we don't read/write a bunch
const MEMOIZATION_RANGE = 30;

export interface LogListProps {
    logsLen: number;
    activeFilter: LogFilter;
    autoScroll: boolean;
    port: number;
}

const LogList = memo((props: LogListProps) => {
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
        if (props.autoScroll) {
            setTimeout(() => {
                listRef.current?.scrollToItem(props.logsLen ?? 0, "end");
            }, 100);
        }
    }, [props.logsLen, logSizes.current]);

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

    const listRef = useRef<VariableSizeList>(null);

    return (
        <div className="log-list">
            <AutoSizer>
                {(size) => (
                    <VariableSizeList
                        ref={listRef}
                        overscanCount={10}
                        itemCount={props.logsLen ?? 0}
                        itemSize={getSize}
                        onItemsRendered={onScroll}
                        {...size}
                    >
                        {({ index, style }) => (
                            <LogLine
                                activeFilter={props.activeFilter}
                                reportSize={reportSize}
                                port={props.port}
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
        </div>
    );
});

export default LogList;
