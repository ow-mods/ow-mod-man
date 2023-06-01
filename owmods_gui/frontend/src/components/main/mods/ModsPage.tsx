import {
    Container,
    Paper,
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableHead,
    TableRow,
    useTheme
} from "@mui/material";
import { ReactNode } from "react";
import ModsToolbar from "./ModsToolbar";
import { useGetTranslation } from "@hooks";

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
    const getTranslation = useGetTranslation();

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
            {props.uniqueNames.length !== 0 ? (
                <TableContainer sx={{ marginTop: theme.spacing(3) }} component={Paper}>
                    <Table>
                        <TableHead>
                            <TableRow>
                                <TableCell>{getTranslation("NAME")}</TableCell>
                                <TableCell width="100px">{getTranslation("DOWNLOADS")}</TableCell>
                                <TableCell width="110px" align="center">
                                    {getTranslation("VERSION")}
                                </TableCell>
                                <TableCell />
                            </TableRow>
                        </TableHead>
                        <TableBody>{props.uniqueNames.map(props.renderRow)}</TableBody>
                    </Table>
                </TableContainer>
            ) : (
                <></>
            )}
        </Container>
    );
};

export default ModsPage;
