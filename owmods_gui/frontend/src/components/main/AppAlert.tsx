import { commands, hooks } from "@commands";
import { withStyledErrorBoundary } from "@components/common/StyledErrorBoundary";
import { useGetTranslation } from "@hooks";
import { ErrorRounded, InfoRounded, LaunchRounded, WarningRounded } from "@mui/icons-material";
import { Box, Link, Button, Palette, Typography, useTheme } from "@mui/material";
import * as shell from "@tauri-apps/plugin-shell";
import { Alert } from "@types";
import { memo } from "react";

type AlertSeverity = "warning" | "error" | "info";

const getColor = (palette: Palette, severity: AlertSeverity) => {
    switch (severity) {
        case "error":
            return palette.error.dark;
        case "warning":
            return palette.secondary.dark;
        default:
            return palette.info.dark;
    }
};

const AlertIcon = (props: { severity: AlertSeverity }) => {
    switch (props.severity) {
        case "error":
            return <ErrorRounded fontSize="small" />;
        case "warning":
            return <WarningRounded fontSize="small" />;
        default:
            return <InfoRounded fontSize="small" />;
    }
};

// const testAlert: Alert = {
//     enabled: true,
//     severity: "error",
//     message: "Test Alert",
//     url: "https://outerwildsmods.com"
// };

const AppAlert = memo(function AppAlert() {
    const getTranslation = useGetTranslation();
    const theme = useTheme();
    const alert: Alert | null = hooks.getAlert("configReload")[1];

    const severity = (alert?.severity ?? "info") as AlertSeverity;

    const onClick = () => {
        shell.open(alert?.url ?? "");
    };

    const onDismiss = () => {
        commands.dismissAlert({ alert: alert! });
    };

    if (alert === null || alert.message?.startsWith("<0.15.3:") || !alert.enabled) {
        return <></>;
    }

    return (
        <Box
            padding={1}
            sx={{
                backgroundColor: getColor(theme.palette, severity)
            }}
        >
            <Typography
                display="flex"
                gap={theme.spacing(0.5)}
                justifyContent="center"
                alignItems="center"
                variant="body2"
            >
                <AlertIcon severity={severity} /> {alert.message}
                {alert.url && (
                    <Button
                        size="small"
                        sx={{ marginLeft: 1 }}
                        color="inherit"
                        startIcon={<LaunchRounded />}
                        onClick={onClick}
                    >
                        {alert.urlLabel ?? "More Info"}
                    </Button>
                )}
                <Link marginLeft={1} color="inherit" href="#" onClick={onDismiss}>
                    {getTranslation("DISMISS")}
                </Link>
            </Typography>
        </Box>
    );
});

export default withStyledErrorBoundary(AppAlert, { justHide: true, resetEvent: "configReload" });
