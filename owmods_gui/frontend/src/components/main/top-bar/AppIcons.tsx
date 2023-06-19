import { DownloadingRounded } from "@mui/icons-material";
import { IconButton } from "@mui/material";
import { ReactNode } from "react";
import OverflowMenu from "./overflow/OverflowMenu";
import ReloadIcon from "./ReloadIcon";
import ODTooltip from "@components/common/ODTooltip";
import SettingsIcon from "./settings/SettingsIcon";

interface AppIconProps {
    label: string;
    children: ReactNode;
    disabled?: boolean;
    onClick?: () => void;
}

export const AppIcon = (props: AppIconProps) => {
    return (
        <ODTooltip title={props.label}>
            <IconButton
                onClick={props.onClick}
                disabled={props.disabled ?? false}
                color="inherit"
                aria-label={props.label}
                sx={{ mr: 2 }}
            >
                {props.children}
            </IconButton>
        </ODTooltip>
    );
};

const AppIcons = () => {
    return (
        <>
            <SettingsIcon />
            <ReloadIcon />
            <AppIcon label="Downloads">
                <DownloadingRounded />
            </AppIcon>
            <OverflowMenu />
        </>
    );
};

export default AppIcons;
