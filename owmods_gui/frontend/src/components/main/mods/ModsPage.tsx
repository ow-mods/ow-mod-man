import { Container, useTheme } from "@mui/material";
import { ReactNode } from "react";
import ModsToolbar from "./ModsToolbar";
import ModsTable from "./ModsTable";

export interface ModsPageProps {
    show: boolean;
    filter: string;
    onFilterChange: (newVal: string) => void;
    uniqueNames: string[];
    renderRow: (uniqueName: string) => ReactNode;
    addToToolbar?: ReactNode;
}

const ModsPage = (props: ModsPageProps) => {
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
                {props.addToToolbar}
            </ModsToolbar>
            {props.uniqueNames.length !== 0 ? <ModsTable {...props} /> : <></>}
        </Container>
    );
};

export default ModsPage;
