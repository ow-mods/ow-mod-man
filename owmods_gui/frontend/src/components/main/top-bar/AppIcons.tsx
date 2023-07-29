import { Box, IconButton, Tooltip, useTheme } from "@mui/material";
import { ReactNode } from "react";
import OverflowMenu from "./overflow/OverflowMenu";
import ReloadIcon from "./ReloadIcon";
import SettingsIcon from "./settings/SettingsIcon";
import DownloadsIcon from "./downloads/DownloadsIcon";

interface AppIconProps {
    label: string;
    children: ReactNode;
    disabled?: boolean;
    onClick?: (e: React.MouseEvent<HTMLButtonElement>) => void;
}

export const AppIcon = (props: AppIconProps) => {
    const comp = (
        <IconButton
            onClick={props.onClick}
            disabled={props.disabled ?? false}
            color="inherit"
            aria-label={props.label}
        >
            {props.children}
        </IconButton>
    );

    if (props.disabled) {
        return comp;
    } else {
        return <Tooltip title={props.label}>{comp}</Tooltip>;
    }
};

const AppIcons = () => {
    const theme = useTheme();

    return (
        <Box display="flex" gap={theme.spacing(2)}>
            <SettingsIcon />
            <ReloadIcon />
            <DownloadsIcon />
            <OverflowMenu />
        </Box>
    );
};

export default AppIcons;
