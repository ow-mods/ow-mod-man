import { hooks } from "@commands";
import { DownloadingRounded } from "@mui/icons-material";
import { Suspense, lazy, memo, useMemo, useState } from "react";
import { AppIcon } from "../AppIcons";
import { Box, CircularProgress, CircularProgressProps, Typography } from "@mui/material";
import { ProgressBar } from "@types";

const DownloadsPopover = lazy(() => import("./DownloadsPopover"));

export const determineProgressVariant = (bar: ProgressBar): CircularProgressProps["variant"] => {
    if (bar.success && bar.progressAction === "Download") {
        // After downloading don't give the wrong idea
        return "indeterminate";
    } else if (bar.progressType === "Indefinite" && bar.success !== undefined) {
        // Show a complete bar if the indefinite action is done
        return "determinate";
    } else {
        return bar.progressType === "Definite" ? "determinate" : "indeterminate";
    }
};

const DownloadsIcon = memo(function DownloadsIcon() {
    const [anchorEl, setAnchorEl] = useState<HTMLButtonElement | null>();
    const downloads = hooks.getDownloads("PROGRESS-UPDATE")[1];

    const sortedDownloads = Object.values(downloads?.bars ?? {});

    sortedDownloads.sort((a, b) => b.position - a.position);

    const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
        setAnchorEl(event.currentTarget);
    };

    const handleClose = () => {
        setAnchorEl(null);
    };

    const openPopover = Boolean(anchorEl);

    const activeDownloads = useMemo(
        () => sortedDownloads.filter((d) => d.success === null || d.progressAction === "Download"),
        [sortedDownloads]
    );

    const len = activeDownloads.length;

    const current = activeDownloads[0];

    return (
        <>
            <Box display="flex" position="relative">
                <Box zIndex={100}>
                    <AppIcon onClick={handleClick} label="Downloads">
                        <DownloadingRounded color={len !== 0 ? "secondary" : "inherit"} />
                    </AppIcon>
                </Box>
                {len !== 0 && (
                    <>
                        <Typography
                            color="secondary"
                            position="absolute"
                            right="-10px"
                            variant="subtitle2"
                            bottom="8px"
                        >
                            {len.toString()}
                        </Typography>
                        {current && (
                            <Box width={30} position="absolute" bottom="-2px" right="0" left="5px">
                                <CircularProgress
                                    size={30}
                                    color="secondary"
                                    value={(current.progress / current.len) * 100}
                                    variant={determineProgressVariant(current)}
                                />
                            </Box>
                        )}
                    </>
                )}
            </Box>
            <Suspense>
                <DownloadsPopover
                    downloads={sortedDownloads}
                    open={openPopover}
                    anchorEl={anchorEl}
                    handleClose={handleClose}
                />
            </Suspense>
        </>
    );
});

export default DownloadsIcon;
