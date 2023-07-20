import { hooks } from "@commands";
import { withStyledErrorBoundary } from "@components/common/StyledErrorBoundary";
import { useGetTranslation } from "@hooks";
import { DeleteRounded } from "@mui/icons-material";
import { Chip, Stack } from "@mui/material";
import { memo, useCallback, useEffect, useRef } from "react";

export interface ModsTagsChipsProps {
    selectedTags: string[];
    onTagsChanged: (newVal: string[]) => void;
}

const tagIsSelected = (selected: string[], tag: string) =>
    selected.find((selectedTag) => selectedTag === tag) !== undefined;

const ModsTagsChips = memo(function ModsTagsChips(props: ModsTagsChipsProps) {
    const availableTags = hooks.getDbTags("remoteRefresh")[1] ?? [];

    const scrollRef = useRef<HTMLDivElement>(null);

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

    useEffect(() => {
        if (scrollRef.current) {
            scrollRef.current.addEventListener("wheel", (event) => {
                if (event.currentTarget) {
                    (event.currentTarget as HTMLDivElement).scrollLeft += event.deltaY;
                }
            });
        }
    }, []);

    return (
        <Stack
            className="scroll-shadows"
            sx={{
                minHeight: "25px",
                overflowX: "auto",
                scrollbarWidth: "none",
                "::-webkit-scrollbar": { display: "none" }
            }}
            direction="row"
            ref={scrollRef}
            gap={1}
        >
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

export default withStyledErrorBoundary(ModsTagsChips, { justHide: true });
