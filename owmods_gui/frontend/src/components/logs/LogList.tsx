import LogLine from "./LogLine";
import AutoSizer from "react-virtualized-auto-sizer";
import { memo, useCallback, useEffect, useRef } from "react";
import { VariableSizeList } from "react-window";
import { LogFilter } from "./LogApp";

export interface LogListProps {
    logLines: number[];
    activeFilter: LogFilter;
    autoScroll: boolean;
    port: number;
}

const LogList = memo((props: LogListProps) => {
    const logLen = props.logLines.length;
    const logSizes = useRef<Record<number, number>>({});

    const reportSize = useCallback((i: number, l: number, size: number) => {
        console.debug(`${l} reported ${size}!`);
        logSizes.current[l] = size;
        listRef.current?.resetAfterIndex(i);
    }, []);
    const getSize = useCallback((i: number) => logSizes.current[i] ?? 40, []);

    useEffect(() => {
        if (props.autoScroll) {
            setTimeout(() => {
                listRef.current?.scrollToItem(logLen, "end");
            }, 100);
        }
    }, [logLen, logSizes.current, props.autoScroll]);

    const listRef = useRef<VariableSizeList>(null);

    return (
        <div className="log-list">
            <AutoSizer>
                {(size) => (
                    <VariableSizeList
                        ref={listRef}
                        overscanCount={0}
                        itemCount={logLen ?? 0}
                        itemSize={getSize}
                        {...size}
                    >
                        {({ index, style }) => (
                            <LogLine
                                reportSize={reportSize}
                                port={props.port}
                                index={index}
                                line={props.logLines[index]}
                                key={index}
                                style={style}
                            />
                        )}
                    </VariableSizeList>
                )}
            </AutoSizer>
        </div>
    );
});

export default LogList;
