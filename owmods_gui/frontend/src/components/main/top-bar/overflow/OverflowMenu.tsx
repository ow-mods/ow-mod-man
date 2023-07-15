import { commands } from "@commands";
import { emit } from "@events";
import { useGetTranslation } from "@hooks";
import {
    BuildRounded,
    FolderOpenRounded,
    HelpRounded,
    MoreHorizRounded
} from "@mui/icons-material";
import { ListItemIcon, ListItemText, MenuItem, Tooltip } from "@mui/material";
import IconButton from "@mui/material/IconButton";
import Menu from "@mui/material/Menu";
import { shell } from "@tauri-apps/api";
import { useState, MouseEvent, useCallback, lazy, Suspense } from "react";

const About = lazy(() => import("./About"));
const Import = lazy(() => import("./Import"));
const Export = lazy(() => import("./Export"));
const InstallFrom = lazy(() => import("./InstallFrom"));

const OverflowMenu = () => {
    const getTranslation = useGetTranslation();
    const [anchorEl, setAnchorEl] = useState<HTMLElement | null>(null);
    const open = Boolean(anchorEl);
    const onClick = useCallback((event: MouseEvent<HTMLButtonElement>) => {
        setAnchorEl(event.currentTarget);
    }, []);
    const onClose = useCallback(() => {
        setAnchorEl(null);
    }, []);

    const onHelp = useCallback(() => {
        shell.open("https://github.com/Bwc9876/ow-mod-man/blob/main/owmods_gui/HELP.md");
        onClose();
    }, [onClose]);

    const onOwmlEdit = useCallback(() => {
        emit("openOwmlSetup", undefined);
        onClose();
    }, [onClose]);

    const onOpenOwml = useCallback(() => {
        commands.openOwml();
    }, []);

    return (
        <>
            <Tooltip title={getTranslation("MORE")}>
                <IconButton
                    edge="end"
                    color="inherit"
                    id="overflow-button"
                    aria-controls={open ? "overflow-menu" : undefined}
                    aria-haspopup="true"
                    aria-expanded={open ? "true" : undefined}
                    onClick={onClick}
                >
                    <MoreHorizRounded />
                </IconButton>
            </Tooltip>
            <Menu
                id="overflow-menu"
                anchorEl={anchorEl}
                open={open}
                keepMounted
                onClose={onClose}
                MenuListProps={{
                    "aria-labelledby": "overflow-button"
                }}
            >
                <Suspense>
                    <InstallFrom onClick={onClose} />
                    <Export onClick={onClose} />
                    <Import onClick={onClose} />
                </Suspense>
                <MenuItem onClick={onOpenOwml}>
                    <ListItemIcon>
                        <FolderOpenRounded fontSize="small" />
                    </ListItemIcon>
                    <ListItemText>{getTranslation("OPEN_OWML")}</ListItemText>
                </MenuItem>
                <MenuItem onClick={onOwmlEdit}>
                    <ListItemIcon>
                        <BuildRounded fontSize="small" />
                    </ListItemIcon>
                    <ListItemText>{getTranslation("EDIT_OWML")}</ListItemText>
                </MenuItem>
                <MenuItem onClick={onHelp}>
                    <ListItemIcon>
                        <HelpRounded fontSize="small" />
                    </ListItemIcon>
                    <ListItemText>{getTranslation("HELP")}</ListItemText>
                </MenuItem>
                <Suspense>
                    <About onClick={onClose} />
                </Suspense>
            </Menu>
        </>
    );
};

export default OverflowMenu;
