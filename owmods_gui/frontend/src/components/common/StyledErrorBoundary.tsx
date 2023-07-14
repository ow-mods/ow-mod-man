import { useGetTranslation } from "@hooks";
import { ErrorRounded } from "@mui/icons-material";
import { Box, Button, Paper, Typography, useTheme } from "@mui/material";
import { ComponentType, ReactNode, useEffect, useMemo } from "react";
import { ErrorBoundary, withErrorBoundary } from "react-error-boundary";
import { TranslationKey } from "./TranslationContext";
import { commands } from "@commands";
import { Event } from "@types";
import { listen } from "@events";

export interface StyledErrorBoundaryProps {
    children: ReactNode;
    center?: boolean;
    errorKey?: TranslationKey;
    resetEvent?: Event["name"];
    justHide?: boolean;
    onFix?: () => void;
    fixButtonKey?: TranslationKey;
}

export const simpleOnError = (err: string) => {
    commands.logError({
        err: err.toString()
    });
};

export const onError = (err: unknown, info: { componentStack: string }) => {
    commands.logError({
        err: `${err?.toString() ?? "null"}\nAt: ${info.componentStack}`
    });
};

const fallback = (options: Omit<StyledErrorBoundaryProps, "children">) =>
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
            if (options.resetEvent) {
                listen(options.resetEvent, () => {
                    if (cancel) return;
                    resetErrorBoundary();
                });
            }
            return () => {
                cancel = true;
            };
        }, [resetErrorBoundary]);

        if (options.justHide) {
            return <></>;
        }

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
                        <ErrorRounded /> {getTranslation(options.errorKey ?? "FATAL_ERROR")}
                    </Box>
                    <Box bgcolor={theme.palette.grey[900]}>
                        <Typography padding={3} variant="body2">
                            {errorString ? errorString : getTranslation("UNKNOWN_ERROR")}
                        </Typography>
                    </Box>
                    {options.onFix && (
                        <Button fullWidth onClick={options.onFix}>
                            {getTranslation(options.fixButtonKey ?? "FIX")}
                        </Button>
                    )}
                </Box>
            </Paper>
        );

        return options.center ? (
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
    const { children, ...options } = props;

    const FallbackComp = useMemo(() => fallback(options), [options]);

    return (
        <ErrorBoundary FallbackComponent={FallbackComp} onError={onError}>
            {children}
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

export const basicFallbackRender = ({ error }: { error: unknown }) => (
    <p
        style={{ color: "rgb(255, 83, 83)", margin: "50vh 10vw", textAlign: "center" }}
    >{`Fatal Error: ${error}`}</p>
);

export default StyledErrorBoundary;
