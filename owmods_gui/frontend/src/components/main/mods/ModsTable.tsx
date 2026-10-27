import { hooks } from "@commands";
import { Paper, Table, TableBody, TableContainer, TableRow, useMediaQuery } from "@mui/material";
import { forwardRef, ReactNode } from "react";
import {
    ScrollerProps,
    TableComponents,
    TableProps,
    TableVirtuoso,
    TableVirtuosoHandle
} from "react-virtuoso";

export interface ModsTableProps {
    uniqueNames: string[];
    renderRow: (uniqueName: string, hideModThumbnails: boolean) => ReactNode;
}

const ScrollerComp = forwardRef<HTMLDivElement, ScrollerProps>(function TScroller(props, ref) {
    return <TableContainer sx={{ flexGrow: 1 }} component={Paper} {...props} ref={ref} />;
});

interface TableColumns {
    showImages: boolean;
}

const TableComp = ({ children, context, style }: TableProps & { context: TableColumns }) => (
    <Table style={{ borderCollapse: "separate", tableLayout: "fixed", ...style }}>
        {context.showImages && (
            <colgroup>
                <col style={{ width: 220 }} />
            </colgroup>
        )}
        {children}
    </Table>
);

const ModsTableComponents = {
    Scroller: ScrollerComp,
    Table: TableComp,
    TableRow,
    TableBody
};

const ModsTable = forwardRef<TableVirtuosoHandle, ModsTableProps>(function ModsTable(
    props: ModsTableProps,
    ref
) {
    const guiConfig = hooks.getGuiConfig("guiConfigReload")[1];

    const imagesEnabled = !(guiConfig?.hideModThumbnails ?? false);
    const enoughWidth = useMediaQuery("(min-width:800px)", {});
    const showImages = imagesEnabled && enoughWidth;

    return (
        <TableVirtuoso
            ref={ref}
            context={{ showImages }}
            components={ModsTableComponents as TableComponents<string, unknown>}
            computeItemKey={(index) => `${index}-${props.uniqueNames[index]}`}
            increaseViewportBy={{ top: 200, bottom: 0 }}
            data={props.uniqueNames}
            itemContent={(_, uniqueName) => props.renderRow(uniqueName, showImages)}
        />
    );
});

export default ModsTable;
