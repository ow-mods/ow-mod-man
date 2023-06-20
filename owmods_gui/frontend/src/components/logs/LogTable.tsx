import { TableCell, useTheme } from "@mui/material";
import { TableContainer, Paper, Table, TableBody, TableHead, TableRow } from "@mui/material";
import { forwardRef, memo } from "react";
import { TableProps, TableVirtuoso } from "react-virtuoso";
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

    return (
        <TableVirtuoso
            components={LogTableComponents}
            computeItemKey={(index) => `${index}-${props.logLines[index][0]}`}
            increaseViewportBy={{ top: 500, bottom: 0 }}
            data={props.logLines}
            fixedHeaderContent={() => (
                <TableRow sx={{ background: theme.palette.grey[900] }}>
                    <TableCell width="150px">{getTranslation("SENDER")}</TableCell>
                    <TableCell>{getTranslation("LOG_MESSAGE")}</TableCell>
                </TableRow>
            )}
            itemContent={(_, data) => <LogRow port={props.port} index={data[0]} count={data[1]} />}
            followOutput
            alignToBottom
        />
    );
});

export default LogTable;
