import { TableCell, useTheme } from "@mui/material";
import { TableContainer, Paper, Table, TableBody, TableHead, TableRow } from "@mui/material";
import { forwardRef, memo, useMemo, useRef } from "react";
import { TableProps, TableVirtuoso, VirtuosoHandle } from "react-virtuoso";
import { LogLines } from "./LogApp";
import { useGetTranslation } from "@hooks";
import LogRow from "./LogRow";

const ScrollerComp = forwardRef<HTMLDivElement>(function TScroller(props, ref) {
    return <TableContainer component={Paper} {...props} ref={ref} />;
});
const TableComp = (props: TableProps) => {
    const theme = useTheme();
    return (
        <Table
            {...props}
            style={{
                backgroundColor: theme.palette.grey[900],
                borderCollapse: "separate",
                tableLayout: "fixed"
            }}
        />
    );
};
const BodyComp = forwardRef<HTMLTableSectionElement>(function TBody(props, ref) {
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
    const theme = useTheme();

    const virtuoso = useRef<VirtuosoHandle>(null);

    const grey = theme.palette.grey[900];

    const header = useMemo(
        () =>
            function HeaderRow() {
                return (
                    <TableRow sx={{ background: grey }}>
                        <TableCell width="150px">{getTranslation("SENDER")}</TableCell>
                        <TableCell>{getTranslation("LOG_MESSAGE")}</TableCell>
                    </TableRow>
                );
            },
        [getTranslation, grey]
    );

    return (
        <TableVirtuoso
            ref={virtuoso}
            components={LogTableComponents}
            computeItemKey={(index) => `${index}-${props.logLines[index]}`}
            increaseViewportBy={500}
            atBottomThreshold={1000}
            data={props.logLines}
            fixedHeaderContent={header}
            itemContent={(_, data) => (
                <LogRow port={props.port} index={data} virtuosoRef={virtuoso} />
            )}
            followOutput
            alignToBottom
        />
    );
});

export default LogTable;
