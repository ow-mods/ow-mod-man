import { useGetTranslation } from "@hooks";
import { HelpRounded, MoreHoriz } from "@mui/icons-material";
import { ListItemIcon, ListItemText, MenuItem, Tooltip } from "@mui/material";
import IconButton from "@mui/material/IconButton";
import Menu from "@mui/material/Menu";
import { shell } from "@tauri-apps/api";
import { useState, MouseEvent, useCallback } from "react";
import About from "./About";
import Import from "./Import";
import Export from "./Export";
import InstallFrom from "./InstallFrom";

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
                    <MoreHoriz />
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
                <InstallFrom onClick={onClose} />
                <Export onClick={onClose} />
                <Import onClick={onClose} />
                <MenuItem onClick={onHelp}>
                    <ListItemIcon>
                        <HelpRounded fontSize="small" />
                    </ListItemIcon>
                    <ListItemText>{getTranslation("HELP")}</ListItemText>
                </MenuItem>
                <About onClick={onClose} />
            </Menu>
        </>
    );
};

export default OverflowMenu;
