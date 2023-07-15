import { Box, CircularProgress, Container, Paper, Typography, useTheme } from "@mui/material";
import { ReactNode, memo, useCallback, useRef } from "react";
import ModsToolbar from "./ModsToolbar";
import ModsTable from "./ModsTable";
import { TableVirtuosoHandle } from "react-virtuoso";

export interface ModsPageProps {
    isLoading: boolean;
    actionsSize: number;
    filter: string;
    noModsText: string;
    onFilterChange: (newVal: string) => void;
    uniqueNames: string[];
    renderRow: (uniqueName: string) => ReactNode;
    children?: ReactNode;
}

const ModsPage = memo(function ModsPage(props: ModsPageProps) {
    const theme = useTheme();

    const virtuosoRef = useRef<TableVirtuosoHandle | null>(null);

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
                padding: theme.spacing(2),
                height: "100%",
                flexDirection: "column"
            }}
            disableGutters
            maxWidth="xl"
        >
            <ModsToolbar filter={props.filter} onFilterChanged={onFilterChange}>
                {props.children}
            </ModsToolbar>
            {props.isLoading ? (
                <Paper sx={{ marginTop: theme.spacing(2), height: "100%" }}>
                    <Box height="100%" display="flex" alignItems="center" justifyContent="center">
                        <CircularProgress color="secondary" />
                    </Box>
                </Paper>
            ) : props.uniqueNames.length !== 0 ? (
                <ModsTable ref={virtuosoRef} {...props} />
            ) : (
                <Paper sx={{ marginTop: theme.spacing(2), height: "100%" }}>
                    <Box height="100%" display="flex" alignItems="center" justifyContent="center">
                        <Typography variant="subtitle1">{props.noModsText}</Typography>
                    </Box>
                </Paper>
            )}
        </Container>
    );
});

export default ModsPage;
