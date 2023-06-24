import { Box, Popover, Typography, useTheme } from "@mui/material";
import { memo } from "react";
import DownloadRow from "./DownloadRow";
import { useGetTranslation } from "@hooks";
import { ProgressBar } from "@types";

export interface DownloadsPopoverProps {
    open: boolean;
    downloads: ProgressBar[];
    handleClose: () => void;
    anchorEl: HTMLElement | null | undefined;
}

const DownloadsPopover = memo(function DownloadsPopover(props: DownloadsPopoverProps) {
    const getTranslation = useGetTranslation();
    const theme = useTheme();

    return (
        <Popover
            id={props.open ? "downloads-popover" : undefined}
            open={props.open}
            anchorEl={props.anchorEl}
            onClose={props.handleClose}
            anchorOrigin={{
                vertical: "bottom",
                horizontal: "left"
            }}
            transformOrigin={{
                vertical: "top",
                horizontal: "left"
            }}
            slotProps={{
                paper: {
                    sx: {
                        maxHeight: "75vh"
                    }
                }
            }}
        >
            <Box
                display="flex"
                flexDirection="column"
                width="50vw"
                gap={theme.spacing(1)}
                padding={theme.spacing(1)}
            >
                {props.downloads.length === 0 ? (
                    <Box height="10vh" display="flex" alignItems="center" justifyContent="center">
                        <Typography variant="subtitle1">
                            {getTranslation("NO_DOWNLOADS")}
                        </Typography>
                    </Box>
                ) : (
                    props.downloads.map((d) => <DownloadRow key={d.id} {...d} />)
                )}
            </Box>
        </Popover>
    );
});

export default DownloadsPopover;
