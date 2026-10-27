import { Skeleton, Tooltip, Typography } from "@mui/material";
import { memo, ReactNode, useLayoutEffect, useRef, useState } from "react";

export const MAX_DESCRIPTION_LINES = 2;
export const DESCRIPTION_LINE_HEIGHT = 1.5;

export interface ModDescriptionProps {
    children: ReactNode;
    isLoading?: boolean;
}

const ModDescription = memo(function ModDescription({ children, isLoading }: ModDescriptionProps) {
    const textRef = useRef<HTMLDivElement>(null);
    const [isOverflowing, setIsOverflowing] = useState(false);

    useLayoutEffect(() => {
        const el = textRef.current;
        if (!el) return;

        const check = () => setIsOverflowing(el.scrollHeight > el.clientHeight + 1);
        check();

        const observer = new ResizeObserver(check);
        observer.observe(el);
        return () => observer.disconnect();
    }, [children, isLoading]);

    if (isLoading) {
        return (
            <Typography
                variant="caption"
                component="div"
                sx={{
                    lineHeight: DESCRIPTION_LINE_HEIGHT,
                    height: `${MAX_DESCRIPTION_LINES * DESCRIPTION_LINE_HEIGHT}em`
                }}
            >
                <Skeleton width={350} height="100%" />
            </Typography>
        );
    }

    return (
        <Tooltip title={children} disableHoverListener={!isOverflowing}>
            <Typography
                ref={textRef}
                variant="caption"
                component="div"
                sx={{
                    lineHeight: DESCRIPTION_LINE_HEIGHT,
                    height: `${MAX_DESCRIPTION_LINES * DESCRIPTION_LINE_HEIGHT}em`,
                    display: "-webkit-box",
                    WebkitBoxOrient: "vertical",
                    WebkitLineClamp: MAX_DESCRIPTION_LINES,
                    overflow: "hidden",
                    overflowWrap: "anywhere"
                }}
            >
                {children}
            </Typography>
        </Tooltip>
    );
});

export default ModDescription;
