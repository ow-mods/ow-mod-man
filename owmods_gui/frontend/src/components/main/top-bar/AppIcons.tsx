import { DownloadingRounded, RefreshRounded, SettingsRounded } from "@mui/icons-material";
import { IconButton } from "@mui/material";
import { ReactNode } from "react";
import OverflowMenu from "./OverflowMenu";

interface AppIconProps {
    label: string;
    children: ReactNode;
}

const AppIcon = (props: AppIconProps) => {
    return (
        <IconButton edge="end" color="inherit" aria-label={props.label} sx={{ mr: 2 }}>
            {props.children}
        </IconButton>
    );
};

const AppIcons = () => {
    return (
        <>
            <AppIcon label="Settings">
                <SettingsRounded />
            </AppIcon>
            <AppIcon label="Reload">
                <RefreshRounded />
            </AppIcon>
            <AppIcon label="Downloads">
                <DownloadingRounded />
            </AppIcon>
            <OverflowMenu />
        </>
    );
};

export default AppIcons;
