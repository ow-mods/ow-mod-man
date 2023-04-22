import { memo, useEffect, useRef } from "react";
import { LogFilter } from "./LogApp";
import { useVirtualizer } from "@tanstack/react-virtual";
import LogLine from "./LogLine";
import ItemMeasurer from "@components/common/ItemMeasurer";

export interface LogListProps {
    port: number;
    logLines: [number, number][];
    activeFilter: LogFilter;
    search: string;
    autoScroll: boolean;
}

const LogList = memo((props: LogListProps) => {
    const parentRef = useRef<HTMLDivElement | null>(null);

    const logLen = props.logLines.length;

    const logVirtualizer = useVirtualizer({
        count: logLen,
        overscan: 75,
        getScrollElement: () => parentRef.current,
        getItemKey: (index) => `${index}-${props.logLines[index][0]}`,
        estimateSize: () => 40
    });

    const items = logVirtualizer.getVirtualItems();

    useEffect(() => {
        if (props.autoScroll && logLen !== 0) {
            logVirtualizer.scrollToIndex(logLen - 1, { align: "end", behavior: "auto" });
        }
    }, [logLen, props.autoScroll]);

    return (
        <div ref={parentRef} className="log-list">
            <div
                style={{
                    height: logVirtualizer.getTotalSize()
                }}
            >
                <div
                    style={{
                        translate: `translateY(${items[0]?.start ?? 0}px)`
                    }}
                >
                    {items.map((item) => (
                        <ItemMeasurer
                            className="log-line"
                            index={item.index}
                            as={LogLine}
                            start={item.start}
                            key={item.key}
                            port={props.port}
                            line={props.logLines[item.index][0]}
                            count={props.logLines[item.index][1]}
                            measure={logVirtualizer.measureElement}
                        />
                    ))}
                </div>
            </div>
        </div>
    );
});

export default LogList;
