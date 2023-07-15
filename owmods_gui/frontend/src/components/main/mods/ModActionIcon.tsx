import ODTooltip from "@components/common/ODTooltip";
import { Box, IconButton, IconButtonProps } from "@mui/material";
import { ReactNode, MouseEvent, memo } from "react";

export interface ModActionIconProps {
    label: string;
    icon: ReactNode;
    onClick?: (e: MouseEvent<HTMLButtonElement>) => void;
    color?: IconButtonProps["color"];
    disabled?: boolean;
    [rest: string | number | symbol]: unknown;
}

const ModActionIcon = memo(function ModActionButton(props: ModActionIconProps) {
    return (
        <ODTooltip title={props.label}>
            <Box height="100%" display="flex" alignItems="center">
                <IconButton
                    disabled={props.disabled}
                    color={props.color}
                    size="small"
                    onClick={props.onClick}
                    {...props}
                >
                    {props.icon}
                </IconButton>
            </Box>
        </ODTooltip>
    );
});

export default ModActionIcon;
