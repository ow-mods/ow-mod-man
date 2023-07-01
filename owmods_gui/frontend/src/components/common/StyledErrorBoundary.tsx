import { useGetTranslation } from "@hooks";
import { ErrorRounded } from "@mui/icons-material";
import { Box, Paper, Typography, useTheme } from "@mui/material";
import { ComponentType, ReactNode, useEffect, useMemo } from "react";
import { ErrorBoundary, withErrorBoundary } from "react-error-boundary";
import { TranslationKey } from "./TranslationContext";
import { commands } from "@commands";
import { listen } from "@tauri-apps/api/event";

export interface StyledErrorBoundaryProps {
    children: ReactNode;
    center?: boolean;
    errorKey?: TranslationKey;
    resetEvent?: string;
}

export const onError = (err: unknown, info: { componentStack: string }) => {
    commands.logError({
        err: `${err?.toString() ?? "null"}\nAt: ${info.componentStack}`
    });
};

const fallback = (
    center?: StyledErrorBoundaryProps["center"],
    errKey?: StyledErrorBoundaryProps["errorKey"],
    resetEvent?: StyledErrorBoundaryProps["resetEvent"]
) =>
    function Fallback({
        error,
        resetErrorBoundary
    }: {
        error: unknown;
        resetErrorBoundary: () => void;
    }) {
        const errorString = error?.toString();
        const getTranslation = useGetTranslation();
        const theme = useTheme();

        useEffect(() => {
            let cancel = false;
            if (resetEvent) {
                listen(resetEvent, () => {
                    if (cancel) return;
                    resetErrorBoundary();
                });
            }
            return () => {
                cancel = true;
            };
        }, [resetErrorBoundary]);

        const text = (
            <Paper
                elevation={3}
                sx={{
                    padding: 3,
                    minWidth: "40%",
                    maxWidth: "60%"
                }}
            >
                <Box gap={2} display="flex" justifyContent="center" flexDirection="column">
                    <Box
                        gap={1}
                        color={theme.palette.error.light}
                        display="flex"
                        flexDirection="row"
                    >
                        <ErrorRounded /> {getTranslation(errKey ?? "FATAL_ERROR")}
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
                display="flex"
                width="100%"
                height="100%"
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
    const FallbackComp = useMemo(
        () => fallback(props.center, props.errorKey, props.resetEvent),
        [props.center, props.errorKey, props.resetEvent]
    );

    return (
        <ErrorBoundary FallbackComponent={FallbackComp} onError={onError}>
            {props.children}
        </ErrorBoundary>
    );
};

export const withStyledErrorBoundary = <Props extends object>(
    component: ComponentType<Props>,
    ...settings: Parameters<typeof fallback>
) =>
    withErrorBoundary(component, {
        FallbackComponent: fallback(...settings),
        onError
    });

export default StyledErrorBoundary;
