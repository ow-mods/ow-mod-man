import { useGetTranslation } from "@hooks";
import { Help, MoreHoriz } from "@mui/icons-material";
import { ListItemIcon, ListItemText, MenuItem } from "@mui/material";
import IconButton from "@mui/material/IconButton";
import Menu from "@mui/material/Menu";
import { useState, MouseEvent } from "react";

const OverflowMenu = () => {
    const getTranslation = useGetTranslation();
    const [anchorEl, setAnchorEl] = useState<HTMLElement | null>(null);
    const open = Boolean(anchorEl);
    const onClick = (event: MouseEvent<HTMLButtonElement>) => {
        setAnchorEl(event.currentTarget);
    };
    const onClose = () => {
        setAnchorEl(null);
    };

    return (
        <div>
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
            <Menu
                id="overflow-menu"
                anchorEl={anchorEl}
                open={open}
                onClose={onClose}
                MenuListProps={{
                    "aria-labelledby": "overflow-button"
                }}
            >
                <MenuItem>
                    <ListItemIcon>
                        <Help fontSize="small" />
                    </ListItemIcon>
                    <ListItemText>{getTranslation("HELP")}</ListItemText>
                </MenuItem>
            </Menu>
        </div>
    );
};

export default OverflowMenu;
