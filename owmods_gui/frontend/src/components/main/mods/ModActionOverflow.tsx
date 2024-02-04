import { CopyAllRounded, MoreVertRounded } from "@mui/icons-material";
import { Menu, MenuItem, ListItemIcon } from "@mui/material";
import {
    ReactNode,
    memo,
    MouseEvent,
    useState,
    useCallback,
    useImperativeHandle,
    forwardRef
} from "react";
import ModActionIcon from "./ModActionIcon";
import { useGetTranslation } from "@hooks";
import clipboard from "@tauri-apps/plugin-clipboard-manager";

export interface ModActionOverflowProps {
    tabId: string;
    uniqueName: string;
    children: ReactNode;
}

export interface ModActionOverflowItemProps {
    label: string;
    icon: ReactNode;
    disabled?: boolean;
    onClose?: () => void;
    onClick?: () => void;
}

export const ModActionOverflowItem = memo(function ModOverflowItem(
    props: ModActionOverflowItemProps
) {
    return (
        <MenuItem
            key={props.label}
            onClick={() => {
                props.onClose?.();
                props.onClick?.();
            }}
            disabled={props.disabled}
        >
            <ListItemIcon>{props.icon}</ListItemIcon>
            {props.label}
        </MenuItem>
    );
});

const ModActionOverflow = forwardRef(function ModActionOverflow(
    props: ModActionOverflowProps,
    ref
) {
    const getTranslation = useGetTranslation();

    const [anchorEl, setAnchorEl] = useState<HTMLElement | null>(null);
    const open = Boolean(anchorEl);
    const onClick = useCallback((event: MouseEvent<HTMLButtonElement>) => {
        setAnchorEl(event.currentTarget);
    }, []);
    const onClose = useCallback(() => {
        setAnchorEl(null);
    }, []);

    const onCopyId = useCallback(() => {
        clipboard.writeText(props.uniqueName);
        onClose();
    }, [props.uniqueName, onClose]);

    useImperativeHandle(ref, () => ({ onClose }), [onClose]);

    const id = `${props.tabId}-${props.uniqueName}`;

    const overflowId = `${id}-actions-overflow`;
    const overflowButtonId = `${id}-actions-overflow-button`;

    return (
        <>
            <ModActionIcon
                onClick={onClick}
                id={overflowButtonId}
                aria-controls={open ? overflowId : undefined}
                aria-haspopup="true"
                aria-expanded={open ? "true" : undefined}
                label={getTranslation("MORE")}
                icon={<MoreVertRounded />}
            />
            <Menu
                id={overflowId}
                anchorEl={anchorEl}
                open={open}
                onClose={onClose}
                MenuListProps={{
                    "aria-labelledby": overflowButtonId
                }}
            >
                {props.children}
                <ModActionOverflowItem
                    label={getTranslation("COPY_UNIQUE_NAME")}
                    icon={<CopyAllRounded />}
                    onClick={onCopyId}
                    onClose={onClose}
                />
            </Menu>
        </>
    );
});

export default memo(ModActionOverflow);
