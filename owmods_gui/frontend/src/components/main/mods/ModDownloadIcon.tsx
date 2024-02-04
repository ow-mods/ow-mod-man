import { hooks } from "@commands";
import { Box, CircularProgress, useTheme } from "@mui/material";
import { ReactNode, memo } from "react";
import ModActionIcon from "./ModActionIcon";
import { determineProgressVariant } from "../top-bar/downloads/DownloadsIcon";

export interface ModDownloadIconProps {
    onClick: () => void;
    tooltip: string;
    icon: ReactNode;
    uniqueName: string;
}

const ModDownloadIcon = memo(function ModDownloadIcon(props: ModDownloadIconProps) {
    const theme = useTheme();

    let busy = hooks.getModBusy("modBusy", { uniqueName: props.uniqueName })[1];
    const bar = hooks.getBarByUniqueName("progressUpdate", { uniqueName: props.uniqueName })[1];

    busy = busy || (bar !== null && (bar?.success ?? undefined) === undefined);

    const percent = bar ? (bar.progress / bar.len) * 100 : 0;

    return busy ? (
        <Box display="flex" alignItems="center">
            <CircularProgress
                sx={{
                    background: theme.palette.background.default,
                    color: theme.palette.secondary.main,
                    borderRadius: "100%",
                    borderWidth: 3,
                    borderStyle: "solid",
                    borderColor: theme.palette.background.default,
                    boxShadow: `0 0 5px 0 ${theme.palette.grey[300]}`
                }}
                variant={bar ? determineProgressVariant(bar) : "indeterminate"}
                color="inherit"
                value={percent}
                size={24}
                thickness={23}
            />
        </Box>
    ) : (
        <ModActionIcon onClick={props.onClick} label={props.tooltip} icon={props.icon} />
    );
});

export default ModDownloadIcon;
