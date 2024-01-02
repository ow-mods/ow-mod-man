import { hooks } from "@commands";
import { useGetTranslation } from "@hooks";
import { ImageRounded } from "@mui/icons-material";
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
import { TableVirtuoso, TableVirtuosoHandle } from "react-virtuoso";

export interface ModsTableProps {
    uniqueNames: string[];
    actionsSize: number;
    renderRow: (uniqueName: string) => ReactNode;
    addToToolbar?: ReactNode;
}

const ScrollerComp = forwardRef<HTMLDivElement>(function TScroller(props, ref) {
    return <TableContainer sx={{ flexGrow: 1 }} component={Paper} {...props} ref={ref} />;
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

const ModsTable = forwardRef<TableVirtuosoHandle, ModsTableProps>(function ModsTable(
    props: ModsTableProps,
    ref
) {
    const getTranslation = useGetTranslation();
    const guiConfig = hooks.getGuiConfig("guiConfigReload")[1];
    const theme = useTheme();

    return (
        <TableVirtuoso
            ref={ref}
            components={ModsTableComponents}
            computeItemKey={(index) => `${index}-${props.uniqueNames[index]}`}
            increaseViewportBy={{ top: 200, bottom: 0 }}
            data={props.uniqueNames}
            fixedHeaderContent={() => (
                <TableRow sx={{ background: theme.palette.grey[900] }}>
                    {guiConfig?.hideModThumbnails || (
                        <TableCell width="220px">
                            <ImageRounded />
                        </TableCell>
                    )}
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
});

export default ModsTable;
