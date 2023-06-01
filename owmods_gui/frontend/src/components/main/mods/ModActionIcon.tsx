import { Box, IconButton, Tooltip } from "@mui/material";
import { ReactNode, MouseEvent, memo } from "react";

export interface ModActionIconProps {
    label: string;
    icon: ReactNode;
    onClick?: (e: MouseEvent<HTMLButtonElement>) => void;
    [rest: string | number | symbol]: unknown;
}

const ModActionIcon = memo(function ModActionButton(props: ModActionIconProps) {
    return (
        <Tooltip title={props.label}>
            <Box display="flex" alignItems="center">
                <IconButton size="small" onClick={props.onClick} {...props}>
                    {props.icon}
                </IconButton>
            </Box>
        </Tooltip>
    );
});

export default ModActionIcon;
