import { IconButton, Tooltip } from "@mui/material";
import { ReactNode, MouseEvent } from "react";

export interface ModActionIconProps {
    label: string;
    icon: ReactNode;
    onClick?: (e: MouseEvent<HTMLButtonElement>) => void;
    [rest: string | number | symbol]: unknown;
}

const ModActionIcon = (props: ModActionIconProps) => {
    return (
        <Tooltip title={props.label}>
            <span style={{ display: "flex", alignItems: "center" }}>
                <IconButton size="small" onClick={props.onClick} {...props}>
                    {props.icon}
                </IconButton>
            </span>
        </Tooltip>
    );
};

export default ModActionIcon;
