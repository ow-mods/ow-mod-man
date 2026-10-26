import { Box, Stack, Toolbar } from "@mui/material";
import FilterInput from "@components/common/FilterInput";
import { ReactNode, memo } from "react";
import { useGetTranslation } from "@hooks";
import ModsTagsFilter from "./ModsTagsFilter";

export interface ModsToolbarProps {
    filter: string;
    onFilterChanged: (newFilter: string) => void;
    selectedTags?: string[];
    hideTags?: string[];
    onSelectedTagsChanged?: (newVal: string[]) => void;
    children?: ReactNode;
}

const ModsToolbar = memo(function GenericModsToolbar(props: ModsToolbarProps) {
    const getTranslation = useGetTranslation();

    return (
        <Box sx={{ display: "flex", flexDirection: "column", gap: 1 }}>
            <Toolbar
                disableGutters
                variant="dense"
                sx={{
                    justifyContent: "space-between",
                    minHeight: 0,
                    gap: 1
                }}
            >
                <Stack direction="row" sx={{ gap: 1, alignItems: "center", minWidth: 0 }}>
                    <FilterInput
                        value={props.filter}
                        onChange={props.onFilterChanged}
                        label={getTranslation("SEARCH")}
                    />
                    {props.selectedTags && props.onSelectedTagsChanged && (
                        <ModsTagsFilter
                            selectedTags={props.selectedTags}
                            hideTags={props.hideTags}
                            onTagsChanged={props.onSelectedTagsChanged}
                        />
                    )}
                </Stack>
                {props.children}
            </Toolbar>
        </Box>
    );
});

export default ModsToolbar;
