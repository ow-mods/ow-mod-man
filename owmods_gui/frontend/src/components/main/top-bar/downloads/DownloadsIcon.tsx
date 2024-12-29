import { hooks } from "@commands";
import { useGetTranslation } from "@hooks";
import { DownloadingRounded } from "@mui/icons-material";
import { lazy, memo, Suspense, useEffect, useMemo, useRef, useState } from "react";
import { AppIcon } from "../AppIcons";
import { Box, CircularProgress, CircularProgressProps, Typography } from "@mui/material";
import { ProgressBar } from "@types";
import { listen } from "@events";

type Timeout = ReturnType<typeof setTimeout>;

const DownloadsPopover = lazy(() => import("./DownloadsPopover"));

export const determineProgressVariant = (bar: ProgressBar): CircularProgressProps["variant"] => {
    if (bar.success && bar.progressAction === "Download") {
        // After downloading don't give the wrong idea
        return "indeterminate";
    } else if (bar.progressType === "Indefinite" && bar.success !== null) {
        // Show a complete bar if the indefinite action is done
        return "determinate";
    } else {
        return bar.progressType === "Definite" ? "determinate" : "indeterminate";
    }
};

type RecentComplete = "none" | "success" | "error";

const recentCompleteClassMap: Record<RecentComplete, string | undefined> = {
    none: undefined,
    success: "downloads-flashing",
    error: "downloads-flashing error"
};

const DownloadsIcon = memo(function DownloadsIcon() {
    const getTranslation = useGetTranslation();

    const [anchorEl, setAnchorEl] = useState<HTMLButtonElement | null>();
    const [recentComplete, setRecentComplete] = useState<RecentComplete>("none");
    const [viewedDownloads, setViewedDownloads] = useState<number>(0);
    const currentTimeout = useRef<Timeout | null>(null);
    const downloads = hooks.getDownloads("progressUpdate")[1];

    const sortedDownloads = Object.values(downloads?.bars ?? {});

    sortedDownloads.sort((a, b) => b.position - a.position);

    const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
        setAnchorEl(event.currentTarget);
        setViewedDownloads(sortedDownloads.filter((d) => d.success !== null).length);
    };

    const handleClose = () => {
        setAnchorEl(null);
    };

    const openPopover = Boolean(anchorEl);

    const activeDownloads = useMemo(
        () => sortedDownloads.filter((d) => d.success === null),
        [sortedDownloads]
    );

    const completeDownloads = useMemo(
        () => sortedDownloads.filter((d) => d.success !== null).length - viewedDownloads,
        [sortedDownloads, viewedDownloads]
    );

    const len = activeDownloads.length;

    const current = activeDownloads[0];

    useEffect(() => {
        const unsubscribe = listen("progressBatchFinish", (hasError) => {
            if (currentTimeout.current) {
                clearTimeout(currentTimeout.current);
            }
            setRecentComplete(hasError ? "error" : "success");
            currentTimeout.current = setTimeout(() => {
                setRecentComplete("none");
            }, 750 * 3); // Animation lasts 700ms and happens 3 times
        });
        return unsubscribe;
    }, []);

    useEffect(() => {
        if (len !== 0) setRecentComplete("none");
        if (currentTimeout.current) {
            clearTimeout(currentTimeout.current);
        }
    }, [len]);

    const iconColor =
        len !== 0
            ? "secondary" // Show as orange when loading
            : "inherit"; // Inherit (white or flash color) otherwise

    return (
        <>
            <Box display="flex" position="relative">
                <Box zIndex={100}>
                    <AppIcon onClick={handleClick} label={getTranslation("DOWNLOADS")}>
                        <DownloadingRounded
                            className={recentCompleteClassMap[recentComplete]}
                            color={iconColor}
                        />
                    </AppIcon>
                </Box>
                {(len !== 0 || completeDownloads !== 0) && (
                    <Typography
                        color={iconColor}
                        className={recentCompleteClassMap[recentComplete]}
                        position="absolute"
                        right="-10px"
                        variant="subtitle2"
                        bottom="8px"
                    >
                        {len === 0 ? completeDownloads.toString() : len.toString()}
                    </Typography>
                )}
                {len !== 0 && current && (
                    <Box width={30} position="absolute" bottom="-2px" right="0" left="5px">
                        <CircularProgress
                            size={30}
                            color="secondary"
                            value={(current.progress / current.len) * 100}
                            variant={determineProgressVariant(current)}
                        />
                    </Box>
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
