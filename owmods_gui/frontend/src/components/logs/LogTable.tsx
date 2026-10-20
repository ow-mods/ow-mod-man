import { TableCell } from "@mui/material";
import { TableContainer, Paper, Table, TableBody, TableHead, TableRow } from "@mui/material";
import { forwardRef, memo, useCallback, useMemo, useRef } from "react";
import {
    ScrollerProps,
    TableBodyProps,
    TableComponents,
    TableProps,
    TableVirtuoso,
    VirtuosoHandle
} from "react-virtuoso";
import { LogLines } from "./LogApp";
import { useGetTranslation } from "@hooks";
import LogRow from "./LogRow";

const ScrollerComp = forwardRef<HTMLDivElement, ScrollerProps>(function TScroller(props, ref) {
    return <TableContainer component={Paper} {...props} ref={ref} />;
});
const TableComp = (props: TableProps) => {
    return (
        <Table
            {...props}
            style={{
                borderCollapse: "separate",
                tableLayout: "fixed"
            }}
        />
    );
};
const BodyComp = forwardRef<HTMLTableSectionElement, TableBodyProps>(function TBody(props, ref) {
    return <TableBody {...props} ref={ref} />;
});

const LogTableComponents = {
    Scroller: ScrollerComp,
    Table: TableComp,
    TableHead: TableHead,
    TableRow: TableRow,
    TableBody: BodyComp
};

export interface LogTableProps {
    port: number;
    logLines: LogLines;
}

const LogTable = memo(function LogTable(props: LogTableProps) {
    const getTranslation = useGetTranslation();

    const virtuoso = useRef<VirtuosoHandle>(null);

    const header = useMemo(
        () =>
            function HeaderRow() {
                return (
                    <TableRow>
                        <TableCell width="150px">{getTranslation("SENDER")}</TableCell>
                        <TableCell>{getTranslation("LOG_MESSAGE")}</TableCell>
                    </TableRow>
                );
            },
        [getTranslation]
    );

    const onRowLoaded = useCallback(() => {
        if (virtuoso.current) {
            virtuoso.current.autoscrollToBottom?.();
        }
    }, []);

    return (
        <TableVirtuoso
            ref={virtuoso}
            components={LogTableComponents as unknown as TableComponents<number, unknown>}
            computeItemKey={(index) => `${index}-${props.logLines[index]}`}
            increaseViewportBy={500}
            atBottomThreshold={1000}
            data={props.logLines}
            fixedHeaderContent={header}
            itemContent={(_, data) => (
                <LogRow port={props.port} index={data} onLoad={onRowLoaded} />
            )}
            followOutput
            alignToBottom
        />
    );
});

export default LogTable;
