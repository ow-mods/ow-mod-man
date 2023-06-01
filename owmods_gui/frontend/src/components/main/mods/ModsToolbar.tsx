import { Paper, Toolbar, useTheme } from "@mui/material";
import FilterInput from "@components/common/FilterInput";
import { FunctionComponent, ReactNode } from "react";
import { useGetTranslation } from "@hooks";

export interface ModsToolbarProps {
    filter: string;
    onFilterChanged: (newFilter: string) => void;
    children?: ReactNode;
}

const ModsToolbar: FunctionComponent<ModsToolbarProps> = ({
    filter,
    onFilterChanged,
    children
}) => {
    const theme = useTheme();
    const getTranslation = useGetTranslation();

    return (
        <Paper
            sx={{
                padding: theme.spacing(1)
            }}
        >
            <Toolbar
                sx={{
                    justifyContent: "space-between",
                    minHeight: 0,
                    padding: 0
                }}
            >
                <FilterInput
                    value={filter}
                    onChange={onFilterChanged}
                    label={getTranslation("SEARCH")}
                />
                {children}
            </Toolbar>
        </Paper>
    );
};

export default ModsToolbar;
