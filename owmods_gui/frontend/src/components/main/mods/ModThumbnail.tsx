import { Box, Skeleton } from "@mui/material";
import { memo, useMemo, useState } from "react";
import ModFallbackThumbnail from "./ModFallbackThumbnail";
import fallBack from "@assets/images/fallback.webp?format=webp&imagetools";
import { hooks } from "@commands";

export interface ModThumbnailProps {
    name: string;
    className?: string;
    uniqueName: string;
    slug?: string;
    isLoading: boolean;
    remoteIsLoading?: boolean;
}

const ModThumbnail = memo(function ModThumbnail(props: ModThumbnailProps) {
    const busy = hooks.getModBusy("modBusy", { uniqueName: props.uniqueName })[1];
    const bar = hooks.getBarByUniqueName("progressUpdate", { uniqueName: props.uniqueName })[1];

    const [imageIsError, setImageIserror] = useState(false);

    const progress = bar ? bar.progress / bar.len : 0;

    const rightBorderRadius = 4;
    const leftBorderRadius = progress === 0 ? rightBorderRadius : 0;

    const thumbnailUrl = useMemo(
        () =>
            props.slug ? `https://ow-mods.github.io/ow-mod-db/thumbnails/${props.slug}.webp` : null,
        [props.slug]
    );

    return (
        <Box position="relative" display="flex" alignItems="center" justifyContent="center">
            {busy && (
                <div
                    className="mod-thumb-cover"
                    style={{
                        width: busy ? `${(1 - progress) * 100}%` : "0",
                        borderTopLeftRadius: leftBorderRadius,
                        borderBottomLeftRadius: leftBorderRadius,
                        borderTopRightRadius: rightBorderRadius,
                        borderBottomRightRadius: rightBorderRadius
                    }}
                />
            )}
            {props.isLoading || props.remoteIsLoading ? (
                <Skeleton variant="rounded" width={450} height={150 / 2} />
            ) : thumbnailUrl === null || imageIsError ? (
                <ModFallbackThumbnail
                    className={props.className}
                    modName={props.name}
                    fallbackUrl={fallBack}
                />
            ) : (
                <img
                    onError={(e) => {
                        e.preventDefault();
                        setImageIserror(true);
                    }}
                    alt={props.name}
                    className={`mod-thumb ${props.className ?? ""}`}
                    width="450"
                    height="150"
                    src={thumbnailUrl}
                />
            )}
        </Box>
    );
});

export default ModThumbnail;
