import { useGetTranslation } from "@hooks";
import { ListItemIcon, Menu, MenuItem } from "@mui/material";
import { memo, useCallback, useState, MouseEvent } from "react";
import {
    AttachMoneyRounded,
    CoffeeRounded,
    FavoriteRounded,
    LocalCafeRounded,
    LocalParkingRounded
} from "@mui/icons-material";
import ModActionIcon from "../ModActionIcon";
import shell from "@tauri-apps/plugin-shell";

export interface LocalModDonateIconProps {
    uniqueName: string;
    links: string[];
}

const determineIcon = (link: string) => {
    const url = new URL(link);
    const host = url.host.replace(/^www\./, "");
    switch (host) {
        case "patreon.com":
            return <LocalParkingRounded />;
        case "buymeacoffee.com":
            return <CoffeeRounded />;
        case "cash.app":
            return <AttachMoneyRounded />;
        case "ko-fi.com":
            return <LocalCafeRounded />;
        default:
            return <FavoriteRounded />;
    }
};

const LocalModDonateIcon = memo(function LocalModDonateIcon(props: LocalModDonateIconProps) {
    const getTranslation = useGetTranslation();

    const [anchorEl, setAnchorEl] = useState<HTMLElement | null>(null);
    const open = Boolean(anchorEl);
    const onClick = useCallback((event: MouseEvent<HTMLButtonElement>) => {
        setAnchorEl(event.currentTarget);
    }, []);
    const onClose = useCallback(() => {
        setAnchorEl(null);
    }, []);

    const overflowId = `${props.uniqueName}-donate-menu`;
    const overflowButtonId = `${props.uniqueName}-donate-button`;

    return (
        <>
            <ModActionIcon
                icon={<FavoriteRounded />}
                onClick={onClick}
                id={overflowButtonId}
                aria-controls={open ? overflowId : undefined}
                aria-haspopup="true"
                aria-expanded={open ? "true" : undefined}
                label={getTranslation("DONATE")}
                disabled={props.links.length === 0}
            />
            <Menu id={overflowId} anchorEl={anchorEl} open={open} onClose={onClose}>
                {props.links.map((link) => (
                    <MenuItem
                        key={link}
                        onClick={() => {
                            shell.open(link);
                            onClose();
                        }}
                    >
                        <ListItemIcon>{determineIcon(link)}</ListItemIcon>
                        {link.replace(/^https?:\/\//, "").replace(/^www\./, "")}
                    </MenuItem>
                ))}
            </Menu>
        </>
    );
});

export default LocalModDonateIcon;
