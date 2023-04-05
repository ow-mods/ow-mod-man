import LogLine from "./LogLine";
import AutoSizer from "react-virtualized-auto-sizer";
import { memo, useCallback, useEffect, useRef } from "react";
import { VariableSizeList } from "react-window";
import { LogFilter } from "./LogApp";

export interface LogListProps {
    port: number;
    logLines: number[];
    activeFilter: LogFilter;
    search: string;
    autoScroll: boolean;
}

const LogList = memo((props: LogListProps) => {
    const logLen = props.logLines.length;
    const logSizes = useRef<Record<number, number>>({});

    const reportSize = useCallback((i: number, l: number, size: number) => {
        logSizes.current[l] = size;
        listRef.current?.resetAfterIndex(i);
    }, []);
    const getSize = useCallback(
        (i: number) => logSizes.current[props.logLines[i]] ?? 40,
        [props.logLines, logSizes.current]
    );

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
            <AutoSizer nonce="MTo3NTM0NTo0MTQ5Nzc5Mjk6MTY4MDU2OTI2Nw==">
                {(size) => (
                    <VariableSizeList
                        ref={listRef}
                        overscanCount={0}
                        itemCount={logLen ?? 0}
                        itemSize={getSize}
                        width={size.width ?? 1000}
                        height={size.height ?? 1000}
                    >
                        {({ index, style }) => (
                            <LogLine
                                port={props.port}
                                reportSize={reportSize}
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
