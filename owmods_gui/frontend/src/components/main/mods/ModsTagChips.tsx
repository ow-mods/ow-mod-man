import { hooks } from "@commands";
import { useGetTranslation } from "@hooks";
import { DeleteRounded } from "@mui/icons-material";
import { Chip, Stack } from "@mui/material";
import { memo, useCallback } from "react";

export interface ModsTagsChipsProps {
    selectedTags: string[];
    onTagsChanged: (newVal: string[]) => void;
}

const tagIsSelected = (selected: string[], tag: string) =>
    selected.find((selectedTag) => selectedTag === tag) !== undefined;

const ModsTagsChips = memo(function ModsTagsChips(props: ModsTagsChipsProps) {
    const availableTags = hooks.getDbTags("remoteRefresh")[1] ?? [];

    const getTranslation = useGetTranslation();

    const { selectedTags, onTagsChanged } = props;

    const onChipClicked = useCallback(
        (tag: string) => {
            if (tagIsSelected(selectedTags, tag)) {
                onTagsChanged(selectedTags.filter((t) => tag !== t));
            } else {
                onTagsChanged([...selectedTags, tag]);
            }
        },
        [onTagsChanged, selectedTags]
    );

    return (
        <Stack direction="row" gap={1}>
            {selectedTags.length !== 0 && (
                <Chip
                    icon={<DeleteRounded />}
                    variant="outlined"
                    color="secondary"
                    label={getTranslation("CLEAR_TAGS")}
                    size="small"
                    onClick={() => props.onTagsChanged([])}
                />
            )}
            {availableTags.map((t) => (
                <Chip
                    label={t}
                    key={t}
                    onClick={() => onChipClicked(t)}
                    size="small"
                    variant={tagIsSelected(props.selectedTags, t) ? "filled" : "outlined"}
                />
            ))}
        </Stack>
    );
});

export default ModsTagsChips;
