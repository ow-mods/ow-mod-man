import { commands } from "@commands";
import { useGetTranslation } from "@hooks";
import { ErrorRounded } from "@mui/icons-material";
import { Box, Paper, Typography, useTheme } from "@mui/material";
import { ReactNode, useCallback, useMemo } from "react";
import { ErrorBoundary } from "react-error-boundary";

export interface StyledErrorBoundaryProps {
    children: ReactNode;
    center?: boolean;
    errorText?: string;
}

const fallback = (
    center: StyledErrorBoundaryProps["center"],
    errText?: StyledErrorBoundaryProps["errorText"]
) =>
    function Fallback({ error }: { error: unknown }) {
        const errorString = error?.toString();
        const getTranslation = useGetTranslation();
        const theme = useTheme();

        const text = (
            <Paper elevation={3} sx={{ padding: 3, minWidth: "40%", maxWidth: "60%" }}>
                <Box gap={2} display="flex" justifyContent="center" flexDirection="column">
                    <Box
                        gap={1}
                        color={theme.palette.error.light}
                        display="flex"
                        flexDirection="row"
                    >
                        <ErrorRounded /> {errText ?? getTranslation("FATAL_ERROR")}
                    </Box>
                    <Box bgcolor={theme.palette.grey[900]}>
                        <Typography padding={3} variant="body2">
                            {errorString ? errorString : getTranslation("UNKNOWN_ERROR")}
                        </Typography>
                    </Box>
                </Box>
            </Paper>
        );

        return center ? (
            <Box
                width="100%"
                height="100%"
                display="flex"
                alignItems="center"
                justifyContent="center"
                padding={5}
            >
                {text}
            </Box>
        ) : (
            text
        );
    };

const StyledErrorBoundary = (props: StyledErrorBoundaryProps) => {
    const fallbackRender = useMemo(
        () => fallback(props.center, props.errorText),
        [props.center, props.errorText]
    );

    const onError = useCallback((err: unknown, info: { componentStack: string }) => {
        commands.logError({
            err: `${err?.toString() ?? "null"}\nAt: ${info.componentStack}`
        });
    }, []);

    return (
        <ErrorBoundary FallbackComponent={fallbackRender} onError={onError}>
            {props.children}
        </ErrorBoundary>
    );
};

export default StyledErrorBoundary;
