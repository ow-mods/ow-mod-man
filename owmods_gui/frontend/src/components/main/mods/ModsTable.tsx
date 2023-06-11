import { useGetTranslation } from "@hooks";
import {
    Paper,
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableHead,
    TableProps,
    TableRow,
    useTheme
} from "@mui/material";
import { ReactNode, forwardRef } from "react";
import { TableVirtuoso } from "react-virtuoso";

export interface ModsTableProps {
    uniqueNames: string[];
    actionsSize: number;
    renderRow: (uniqueName: string) => ReactNode;
    addToToolbar?: ReactNode;
}

const ScrollerComp = forwardRef<HTMLDivElement>(function TScroller(props, ref) {
    const theme = useTheme();
    return (
        <TableContainer
            sx={{ marginTop: theme.spacing(3) }}
            component={Paper}
            {...props}
            ref={ref}
        />
    );
});
const TableComp = (props: TableProps) => (
    <Table {...props} style={{ borderCollapse: "separate", tableLayout: "fixed" }} />
);
const BodyComp = forwardRef<HTMLTableSectionElement>(function TBody(props, ref) {
    return <TableBody {...props} ref={ref} />;
});

const ModsTableComponents = {
    Scroller: ScrollerComp,
    Table: TableComp,
    TableHead: TableHead,
    TableRow: TableRow,
    TableBody: BodyComp
};

const ModsTable = (props: ModsTableProps) => {
    const getTranslation = useGetTranslation();
    const theme = useTheme();

    return (
        <TableVirtuoso
            components={ModsTableComponents}
            computeItemKey={(index) => `${index}-${props.uniqueNames[index]}`}
            increaseViewportBy={1000}
            data={props.uniqueNames}
            fixedHeaderContent={() => (
                <TableRow sx={{ background: theme.palette.grey[900] }}>
                    <TableCell>{getTranslation("NAME")}</TableCell>
                    <TableCell width="100px">{getTranslation("DOWNLOADS")}</TableCell>
                    <TableCell width="110px" align="center">
                        {getTranslation("VERSION")}
                    </TableCell>
                    <TableCell width={props.actionsSize} align="center">
                        {getTranslation("ACTIONS")}
                    </TableCell>
                </TableRow>
            )}
            itemContent={(_, uniqueName) => props.renderRow(uniqueName)}
        />
    );
};

export default ModsTable;
