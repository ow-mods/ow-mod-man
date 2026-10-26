import { Box } from "@mui/material";
import { memo, ReactNode } from "react";

export interface ModDetailItemProps {
    icon: ReactNode;
    children: ReactNode;
}

const ModDetailItem = memo(function ModDetailItem(props: ModDetailItemProps) {
    return (
        <Box sx={{ display: "flex", alignItems: "center", gap: 0.1 }}>
            {props.icon}
            {props.children}
        </Box>
    );
});

export default ModDetailItem;
