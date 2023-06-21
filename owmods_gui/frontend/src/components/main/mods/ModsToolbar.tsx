import { Paper, Toolbar, useTheme } from "@mui/material";
import FilterInput from "@components/common/FilterInput";
import { ReactNode, memo } from "react";
import { useGetTranslation } from "@hooks";

export interface ModsToolbarProps {
    filter: string;
    onFilterChanged: (newFilter: string) => void;
    children?: ReactNode;
}

const ModsToolbar = memo(function GenericModsToolbar(props: ModsToolbarProps) {
    const theme = useTheme();
    const getTranslation = useGetTranslation();

    return (
        <Paper sx={{ padding: 1 }}>
            <Toolbar
                disableGutters
                variant="dense"
                sx={{
                    justifyContent: "space-between",
                    minHeight: 0
                }}
            >
                <FilterInput
                    value={props.filter}
                    onChange={props.onFilterChanged}
                    label={getTranslation("SEARCH")}
                />
                {props.children}
            </Toolbar>
        </Paper>
    );
});

export default ModsToolbar;
