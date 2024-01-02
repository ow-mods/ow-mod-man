import { Box, Skeleton } from "@mui/material";
import { memo, useState } from "react";
import ModFallbackThumbnail from "./ModFallbackThumbnail";
import fallBack from "@assets/images/fallback.webp?format=webp&imagetools";
import { hooks } from "@commands";

export interface ModThumbnailProps {
    name: string;
    className?: string;
    url?: string;
    uniqueName: string;
    isLoading: boolean;
    remoteIsLoading?: boolean;
}

const modDbThumbnailUrl = "https://ow-mods.github.io/ow-mod-db/thumbnails";

const ModThumbnail = memo(function ModThumbnail(props: ModThumbnailProps) {
    let busy = hooks.getModBusy("modBusy", { uniqueName: props.uniqueName })[1];
    const bar = hooks.getBarByUniqueName("progressUpdate", { uniqueName: props.uniqueName })[1];

    busy = busy || (bar !== null && (bar?.success ?? undefined) === undefined);

    const [imageIsError, setImageIserror] = useState(false);

    const progress = bar ? bar.progress / bar.len : 0;

    const rightBorderRadius = 4;
    const leftBorderRadius = progress === 0 ? rightBorderRadius : 0;

    return (
        <Box position="relative" display="flex" alignItems="center" justifyContent="center">
            {busy && (
                <div
                    className="mod-thumb-cover"
                    style={{
                        width: busy && progress !== 0 ? `${(1 - progress) * 100}%` : "0",
                        borderTopLeftRadius: leftBorderRadius,
                        borderBottomLeftRadius: leftBorderRadius,
                        borderTopRightRadius: rightBorderRadius,
                        borderBottomRightRadius: rightBorderRadius
                    }}
                />
            )}
            {props.isLoading || props.remoteIsLoading ? (
                <Skeleton variant="rounded" width={450} height={150 / 2} />
            ) : (props.url ?? null) === null || imageIsError ? (
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
                    src={`${modDbThumbnailUrl}/${props.url}`}
                />
            )}
        </Box>
    );
});

export default ModThumbnail;
