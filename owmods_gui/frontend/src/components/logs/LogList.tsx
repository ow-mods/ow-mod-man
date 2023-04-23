import { memo } from "react";
import { LogFilter } from "./LogApp";
import LogLine from "./LogLine";
import { Virtuoso } from "react-virtuoso";

export interface LogListProps {
    port: number;
    logLines: [number, number][];
    activeFilter: LogFilter;
    search: string;
}

const LogList = memo((props: LogListProps) => {
    return (
        <Virtuoso
            className="log-list"
            increaseViewportBy={5000}
            computeItemKey={(index) => `${index}-${props.logLines[index][0]}`}
            data={props.logLines}
            itemContent={(_, data) => <LogLine port={props.port} line={data[0]} count={data[1]} />}
            followOutput
            alignToBottom
        />
    );
});

export default LogList;
