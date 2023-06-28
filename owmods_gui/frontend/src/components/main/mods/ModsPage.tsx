import { Box, CircularProgress, Container, Paper, Typography, useTheme } from "@mui/material";
import { ReactNode, memo } from "react";
import ModsToolbar from "./ModsToolbar";
import ModsTable from "./ModsTable";
import StyledErrorBoundary from "@components/common/StyledErrorBoundary";
import { useGetTranslation } from "@hooks";

export interface ModsPageProps {
    isLoading: boolean;
    actionsSize: number;
    show: boolean;
    filter: string;
    noModsText: string;
    onFilterChange: (newVal: string) => void;
    uniqueNames: string[];
    renderRow: (uniqueName: string) => ReactNode;
    children?: ReactNode;
}

const ModsPage = memo(function ModsPage(props: ModsPageProps) {
    const theme = useTheme();
    const getTranslation = useGetTranslation();

    return (
        <Container
            sx={{
                padding: theme.spacing(2),
                height: "100%",
                display: props.show ? "flex" : "none",
                flexDirection: "column"
            }}
            disableGutters
            maxWidth="xl"
        >
            <StyledErrorBoundary errorText={getTranslation("PAGE_ERROR")} center>
                <ModsToolbar filter={props.filter} onFilterChanged={props.onFilterChange}>
                    {props.children}
                </ModsToolbar>
                {props.isLoading ? (
                    <Paper sx={{ marginTop: theme.spacing(2), height: "100%" }}>
                        <Box
                            height="100%"
                            display="flex"
                            alignItems="center"
                            justifyContent="center"
                        >
                            <CircularProgress color="secondary" />
                        </Box>
                    </Paper>
                ) : props.uniqueNames.length !== 0 ? (
                    <ModsTable {...props} />
                ) : (
                    <Paper sx={{ marginTop: theme.spacing(2), height: "100%" }}>
                        <Box
                            height="100%"
                            display="flex"
                            alignItems="center"
                            justifyContent="center"
                        >
                            <Typography variant="subtitle1">{props.noModsText}</Typography>
                        </Box>
                    </Paper>
                )}
            </StyledErrorBoundary>
        </Container>
    );
});

export default ModsPage;
