import { Box, Tooltip } from "@mui/material";
import { memo, ReactNode, useLayoutEffect, useRef, useState } from "react";

export interface ModDetailItemProps {
    icon: ReactNode;
    children: ReactNode;
    truncate?: boolean;
}

const ModDetailItem = memo(function ModDetailItem(props: ModDetailItemProps) {
    const { truncate } = props;
    const textRef = useRef<HTMLSpanElement>(null);
    const [isOverflowing, setIsOverflowing] = useState(false);

    useLayoutEffect(() => {
        const el = textRef.current;
        if (!el) return;

        const check = () => setIsOverflowing(el.scrollWidth > el.clientWidth);
        check();

        const observer = new ResizeObserver(check);
        observer.observe(el);
        return () => observer.disconnect();
    }, [props.children]);

    return (
        <Box
            sx={{
                display: "flex",
                alignItems: "center",
                gap: 0.1,
                ...(truncate ? { minWidth: 0 } : {})
            }}
        >
            {props.icon}
            {truncate ? (
                <Tooltip title={props.children} disableHoverListener={!isOverflowing}>
                    <Box
                        component="span"
                        ref={textRef}
                        sx={{
                            overflow: "hidden",
                            textOverflow: "ellipsis",
                            whiteSpace: "nowrap"
                        }}
                    >
                        {props.children}
                    </Box>
                </Tooltip>
            ) : (
                props.children
            )}
        </Box>
    );
});

export default ModDetailItem;
