import { hooks } from "@commands";
import ODTooltip from "@components/common/ODTooltip";
import { useGetTranslation } from "@hooks";
import { Download as DownloadsIcon, ImageRounded } from "@mui/icons-material";
import {
    Box,
    Paper,
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableHead,
    TableProps,
    TableRow,
    TableRowProps,
    useMediaQuery,
    useTheme
} from "@mui/material";
import { forwardRef, ReactNode } from "react";
import {
    ScrollerProps,
    TableBodyProps,
    TableComponents,
    TableVirtuoso,
    TableVirtuosoHandle
} from "react-virtuoso";

export interface ModsTableProps {
    uniqueNames: string[];
    actionsSize: number;
    renderRow: (uniqueName: string, hideModThumbnails: boolean) => ReactNode;
    addToToolbar?: ReactNode;
}

const ScrollerComp = forwardRef<HTMLDivElement, ScrollerProps>(function TScroller(props, ref) {
    return <TableContainer sx={{ flexGrow: 1 }} component={Paper} {...props} ref={ref} />;
});
const TableComp = (props: TableProps) => (
    <Table {...props} style={{ borderCollapse: "separate", tableLayout: "fixed" }} />
);
const BodyComp = forwardRef<HTMLTableSectionElement, TableBodyProps>(function TBody(props, ref) {
    return <TableBody {...props} ref={ref} />;
});
const RowComp = forwardRef<HTMLTableRowElement, TableRowProps>(function TRow(props, ref) {
    return <TableRow {...props} ref={ref} />;
});

const ModsTableComponents = {
    Scroller: ScrollerComp,
    Table: TableComp,
    TableHead: TableHead,
    TableRow: RowComp,
    TableBody: BodyComp
};

const ModsTable = forwardRef<TableVirtuosoHandle, ModsTableProps>(function ModsTable(
    props: ModsTableProps,
    ref
) {
    const getTranslation = useGetTranslation();
    const guiConfig = hooks.getGuiConfig("guiConfigReload")[1];
    const theme = useTheme();

    const imagesEnabled = !(guiConfig?.hideModThumbnails ?? false);
    const enoughWidth = useMediaQuery("(min-width:800px)", {});
    const showImages = imagesEnabled && enoughWidth;

    return (
        <TableVirtuoso
            ref={ref}
            components={ModsTableComponents as TableComponents<string, unknown>}
            computeItemKey={(index) => `${index}-${props.uniqueNames[index]}`}
            increaseViewportBy={{ top: 200, bottom: 0 }}
            data={props.uniqueNames}
            fixedHeaderContent={() => (
                <TableRow sx={{ background: theme.palette.grey[900] }}>
                    <>
                        {showImages && (
                            <TableCell width="220px">
                                <Box display="flex" alignItems="center">
                                    <ImageRounded />
                                </Box>
                            </TableCell>
                        )}
                        <TableCell>{getTranslation("NAME")}</TableCell>
                        <TableCell width="50px" align="right">
                            <ODTooltip title={getTranslation("DOWNLOAD_COUNT")}>
                                <Box display="flex" alignItems="center">
                                    <DownloadsIcon />
                                </Box>
                            </ODTooltip>
                        </TableCell>
                        <TableCell width="110px" align="center">
                            {getTranslation("VERSION")}
                        </TableCell>
                        <TableCell width={props.actionsSize} align="center">
                            {getTranslation("ACTIONS")}
                        </TableCell>
                    </>
                </TableRow>
            )}
            itemContent={(_, uniqueName) => props.renderRow(uniqueName, showImages)}
        />
    );
});

export default ModsTable;
