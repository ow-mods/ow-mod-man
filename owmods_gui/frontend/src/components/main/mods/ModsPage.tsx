import { Box, CircularProgress, Container, Paper, Typography, useTheme } from "@mui/material";
import { ReactNode, memo } from "react";
import ModsToolbar from "./ModsToolbar";
import ModsTable from "./ModsTable";
import { useGetTranslation } from "@hooks";

export interface ModsPageProps {
    isLoading: boolean;
    show: boolean;
    filter: string;
    onFilterChange: (newVal: string) => void;
    uniqueNames: string[];
    renderRow: (uniqueName: string) => ReactNode;
    children?: ReactNode;
}

const ModsPage = memo(function ModsPage(props: ModsPageProps) {
    const getTranslation = useGetTranslation();
    const theme = useTheme();

    return (
        <Container
            sx={{
                paddingTop: theme.spacing(3),
                paddingBottom: theme.spacing(3),
                height: "100%",
                display: props.show ? "flex" : "none",
                flexDirection: "column"
            }}
            maxWidth="xl"
        >
            <ModsToolbar filter={props.filter} onFilterChanged={props.onFilterChange}>
                {props.children}
            </ModsToolbar>
            {props.isLoading ? (
                <Paper sx={{ marginTop: theme.spacing(3), height: "100%" }}>
                    <Box height="100%" display="flex" alignItems="center" justifyContent="center">
                        <CircularProgress color="secondary" />
                    </Box>
                </Paper>
            ) : props.uniqueNames.length !== 0 ? (
                <ModsTable {...props} />
            ) : (
                <Paper sx={{ marginTop: theme.spacing(3), height: "100%" }}>
                    <Box height="100%" display="flex" alignItems="center" justifyContent="center">
                        <Typography variant="subtitle1">{getTranslation("NO_MODS")}</Typography>
                    </Box>
                </Paper>
            )}
        </Container>
    );
});

export default ModsPage;
