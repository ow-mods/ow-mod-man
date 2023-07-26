import { hooks } from "@commands";
import { withStyledErrorBoundary } from "@components/common/StyledErrorBoundary";
import { ErrorRounded, InfoRounded, LaunchRounded, WarningRounded } from "@mui/icons-material";
import { Box, Button, Palette, Typography, useTheme } from "@mui/material";
import { shell } from "@tauri-apps/api";
import { Alert } from "@types";
import { memo, useCallback } from "react";

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
    const theme = useTheme();
    const alert: Alert | null = hooks.getAlert("configReload")[1];

    const severity = (alert?.severity ?? "info") as AlertSeverity;

    const onClick = useCallback(() => {
        shell.open(alert?.url ?? "");
    }, [alert?.url]);

    if (alert === null || !alert.enabled) {
        return <></>;
    }

    return (
        <Box
            padding={theme.spacing(1)}
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
                        sx={{ marginLeft: theme.spacing(1) }}
                        color="inherit"
                        startIcon={<LaunchRounded />}
                        onClick={onClick}
                    >
                        {alert.urlLabel ?? "More Info"}
                    </Button>
                )}
            </Typography>
        </Box>
    );
});

export default withStyledErrorBoundary(AppAlert, { justHide: true, resetEvent: "configReload" });
