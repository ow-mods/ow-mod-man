import ODTooltip from "@components/common/ODTooltip";
import { useGetTranslation } from "@hooks";
import { Star, Help } from "@mui/icons-material";
import { Chip, Stack } from "@mui/material";
import { memo } from "react";

export interface RecommendedModBadgeProps {
    recommendedBy: string[];
}

const RecommendedModBadge = memo(function RecommendedModBadge(props: RecommendedModBadgeProps) {
    const getTranslation = useGetTranslation();

    const tooltip = getTranslation("RECOMMENDED_BECAUSE").replaceAll(
        "$mods$",
        props.recommendedBy.join(", ")
    );

    return (
        <Stack direction="row">
            <Chip
                icon={<Star fontSize="inherit" />}
                label={getTranslation("RECOMMENDED")}
                size="small"
                color="primary"
            />
            <ODTooltip title={tooltip}>
                <Help color="action" fontSize="inherit" />
            </ODTooltip>
        </Stack>
    );
});

export default RecommendedModBadge;
