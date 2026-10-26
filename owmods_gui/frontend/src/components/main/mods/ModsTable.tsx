import { hooks } from "@commands";
import {
    Paper,
    Table,
    TableBody,
    TableContainer,
    TableProps,
    TableRow,
    TableRowProps,
    useMediaQuery
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
}

const ScrollerComp = forwardRef<HTMLDivElement, ScrollerProps>(function TScroller(props, ref) {
    return <TableContainer sx={{ flexGrow: 1 }} component={Paper} {...props} ref={ref} />;
});

interface TableColumns {
    showImages: boolean;
    actionsSize: number;
}

const TableComp = ({ children, context, ...props }: TableProps & { context: TableColumns }) => {
    const { showImages, actionsSize } = context;

    return (
        <Table {...props} style={{ borderCollapse: "separate", tableLayout: "fixed" }}>
            <colgroup>
                {showImages && <col style={{ width: 220 }} />}
                <col />
                <col style={{ width: actionsSize }} />
            </colgroup>
            {children}
        </Table>
    );
};
const BodyComp = forwardRef<HTMLTableSectionElement, TableBodyProps>(function TBody(props, ref) {
    return <TableBody {...props} ref={ref} />;
});
const RowComp = forwardRef<HTMLTableRowElement, TableRowProps>(function TRow(props, ref) {
    return <TableRow {...props} ref={ref} />;
});

const ModsTableComponents = {
    Scroller: ScrollerComp,
    Table: TableComp,
    TableRow: RowComp,
    TableBody: BodyComp
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
            context={{ showImages, actionsSize: props.actionsSize }}
            components={ModsTableComponents as TableComponents<string, unknown>}
            computeItemKey={(index) => `${index}-${props.uniqueNames[index]}`}
            increaseViewportBy={{ top: 200, bottom: 0 }}
            data={props.uniqueNames}
            itemContent={(_, uniqueName) => props.renderRow(uniqueName, showImages)}
        />
    );
});

export default ModsTable;
