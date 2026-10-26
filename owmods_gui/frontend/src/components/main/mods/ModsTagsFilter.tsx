import { hooks } from "@commands";
import { withStyledErrorBoundary } from "@components/common/StyledErrorBoundary";
import { useGetTranslation } from "@hooks";
import { CloseRounded, DeleteRounded, FilterListRounded } from "@mui/icons-material";
import { Badge, Box, Button, ButtonGroup, Chip, Popover } from "@mui/material";
import { MouseEvent, memo, useCallback, useState } from "react";

export interface ModsTagsFilterProps {
    selectedTags: string[];
    hideTags?: string[];
    onTagsChanged: (newVal: string[]) => void;
}

const ModsTagsFilter = memo(function ModsTagsFilter(props: ModsTagsFilterProps) {
    const availableTags = hooks.getDbTags("remoteRefresh")[1] ?? [];

    const getTranslation = useGetTranslation();

    const { selectedTags, hideTags, onTagsChanged } = props;

    const [anchorEl, setAnchorEl] = useState<HTMLButtonElement | null>(null);

    const open = Boolean(anchorEl);

    const onOpen = useCallback((event: MouseEvent<HTMLButtonElement>) => {
        setAnchorEl(event.currentTarget);
    }, []);

    const onClose = useCallback(() => {
        setAnchorEl(null);
    }, []);

    const onTagClicked = useCallback(
        (tag: string) => {
            if (selectedTags.includes(tag)) {
                onTagsChanged(selectedTags.filter((t) => t !== tag));
            } else {
                onTagsChanged([...selectedTags, tag]);
            }
        },
        [onTagsChanged, selectedTags]
    );

    return (
        <>
            <ButtonGroup size="small">
                <Button
                    color="neutral"
                    startIcon={
                        <Badge color="primary" variant="dot" invisible={selectedTags.length === 0}>
                            <FilterListRounded />
                        </Badge>
                    }
                    onClick={onOpen}
                >
                    Tags
                </Button>
                {selectedTags.length > 0 && (
                    <Button
                        color="neutral"
                        aria-label={getTranslation("CLEAR_TAGS")}
                        onClick={() => onTagsChanged([])}
                        sx={{ minWidth: 0, paddingX: 1 }}
                    >
                        <CloseRounded fontSize="small" />
                    </Button>
                )}
            </ButtonGroup>
            <Popover
                open={open}
                anchorEl={anchorEl}
                onClose={onClose}
                anchorOrigin={{ vertical: "bottom", horizontal: "left" }}
                transformOrigin={{ vertical: "top", horizontal: "left" }}
                slotProps={{
                    paper: {
                        sx: {
                            marginTop: 1,
                            maxWidth: "min(90vw, 400px)"
                        }
                    }
                }}
            >
                <Box
                    sx={{
                        display: "flex",
                        flexWrap: "wrap",
                        gap: 1,
                        padding: 1.5,
                        maxHeight: "50vh",
                        overflowY: "auto"
                    }}
                >
                    {availableTags.map((tag) => {
                        const hidden = hideTags?.includes(tag) ?? false;
                        return (
                            <Chip
                                key={tag}
                                label={tag}
                                size="small"
                                disabled={hidden}
                                onClick={() => onTagClicked(tag)}
                                color={
                                    !hidden && selectedTags.includes(tag) ? "primary" : "default"
                                }
                                sx={{ textDecoration: hidden ? "line-through" : "none" }}
                            />
                        );
                    })}
                    {selectedTags.length > 0 && (
                        <Chip
                            icon={<DeleteRounded />}
                            label={getTranslation("CLEAR_TAGS")}
                            size="small"
                            variant="outlined"
                            color="secondary"
                            onClick={() => onTagsChanged([])}
                        />
                    )}
                </Box>
            </Popover>
        </>
    );
});

export default withStyledErrorBoundary(ModsTagsFilter, { justHide: true });
