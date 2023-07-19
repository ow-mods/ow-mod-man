import { Paper, Toolbar } from "@mui/material";
import FilterInput from "@components/common/FilterInput";
import { ReactNode, memo } from "react";
import { useGetTranslation } from "@hooks";
import ModsTagsChips from "./ModsTagChips";

export interface ModsToolbarProps {
    filter: string;
    onFilterChanged: (newFilter: string) => void;
    selectedTags?: string[];
    onSelectedTagsChanged?: (newVal: string[]) => void;
    children?: ReactNode;
}

const ModsToolbar = memo(function GenericModsToolbar(props: ModsToolbarProps) {
    const getTranslation = useGetTranslation();

    return (
        <Paper sx={{ padding: 1, display: "flex", flexDirection: "column", gap: 1 }}>
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
            {props.selectedTags && props.onSelectedTagsChanged && (
                <ModsTagsChips
                    selectedTags={props.selectedTags}
                    onTagsChanged={props.onSelectedTagsChanged}
                />
            )}
        </Paper>
    );
});

export default ModsToolbar;
