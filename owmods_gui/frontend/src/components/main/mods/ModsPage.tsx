import { Box, CircularProgress, Container, Paper, Typography } from "@mui/material";
import { ReactNode, memo, useCallback, useRef } from "react";
import ModsToolbar from "./ModsToolbar";
import ModsTable from "./ModsTable";
import { TableVirtuosoHandle } from "react-virtuoso";
import { useGetTranslation } from "@hooks";

export interface ModsPageProps {
    isLoading: boolean;
    actionsSize: number;
    filter: string;
    noModsText: string;
    onFilterChange: (newVal: string) => void;
    selectedTags?: string[];
    onSelectedTagsChanged?: (newVal: string[]) => void;
    uniqueNames: string[];
    renderRow: (uniqueName: string) => ReactNode;
    children?: ReactNode;
}

const ModsPage = memo(function ModsPage(props: ModsPageProps) {
    const virtuosoRef = useRef<TableVirtuosoHandle | null>(null);

    const getTranslation = useGetTranslation();

    const propFilterChange = props.onFilterChange;

    const onFilterChange = useCallback(
        (newVal: string) => {
            virtuosoRef.current?.scrollTo({ top: 0 });
            propFilterChange(newVal);
        },
        [propFilterChange]
    );

    return (
        <Container
            sx={{
                padding: 2,
                display: "flex",
                height: "100%",
                flexDirection: "column",
                gap: 2
            }}
            disableGutters
            maxWidth="xl"
        >
            <ModsToolbar
                selectedTags={props.selectedTags}
                onSelectedTagsChanged={props.onSelectedTagsChanged}
                filter={props.filter}
                onFilterChanged={onFilterChange}
            >
                {props.children}
            </ModsToolbar>
            {props.isLoading ? (
                <Paper sx={{ flexGrow: 1, height: "100%" }}>
                    <Box height="100%" display="flex" alignItems="center" justifyContent="center">
                        <CircularProgress color="secondary" />
                    </Box>
                </Paper>
            ) : props.uniqueNames.length !== 0 ? (
                <ModsTable ref={virtuosoRef} {...props} />
            ) : (
                <Paper sx={{ flexGrow: 1, height: "100%" }}>
                    <Box height="100%" display="flex" alignItems="center" justifyContent="center">
                        <Typography variant="subtitle1">
                            {props.filter === "" && props.selectedTags?.length === 0
                                ? props.noModsText
                                : getTranslation("NO_MODS_FROM_FILTER")}
                        </Typography>
                    </Box>
                </Paper>
            )}
        </Container>
    );
});

export default ModsPage;
