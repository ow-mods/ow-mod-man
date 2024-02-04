import { commands } from "@commands";
import { emit } from "@events";
import { useGetTranslation } from "@hooks";
import {
    BuildRounded,
    FavoriteRounded,
    FolderOpenRounded,
    HelpRounded,
    MoreHorizRounded,
    ReceiptRounded
} from "@mui/icons-material";
import { ListItemIcon, ListItemText, MenuItem, Tooltip } from "@mui/material";
import IconButton from "@mui/material/IconButton";
import Menu from "@mui/material/Menu";
import * as shell from "@tauri-apps/plugin-shell";
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
        shell.open("https://github.com/ow-mods/ow-mod-man/blob/main/owmods_gui/HELP.md");
        onClose();
    }, [onClose]);

    const onOwmlEdit = useCallback(() => {
        emit("openOwmlSetup", undefined);
        onClose();
    }, [onClose]);

    const onOpenOwml = useCallback(() => {
        commands.openOwml();
        onClose();
    }, [onClose]);

    const onLogs = useCallback(() => {
        commands.showLogsFolder();
        onClose();
    }, [onClose]);

    const onDonate = useCallback(() => {
        shell.open("https://paypal.me/Bwc9876");
        onClose();
    }, [onClose]);

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
                <MenuItem onClick={onLogs}>
                    <ListItemIcon>
                        <ReceiptRounded fontSize="small" />
                    </ListItemIcon>
                    <ListItemText>{getTranslation("LOGS")}</ListItemText>
                </MenuItem>
                <MenuItem onClick={onHelp}>
                    <ListItemIcon>
                        <HelpRounded fontSize="small" />
                    </ListItemIcon>
                    <ListItemText>{getTranslation("HELP")}</ListItemText>
                </MenuItem>
                <MenuItem onClick={onDonate}>
                    <ListItemIcon>
                        <FavoriteRounded fontSize="small" />
                    </ListItemIcon>
                    <ListItemText>{getTranslation("DONATE")}</ListItemText>
                </MenuItem>
                <Suspense>
                    <About onClick={onClose} />
                </Suspense>
            </Menu>
        </>
    );
};

export default OverflowMenu;
