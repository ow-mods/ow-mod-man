import { hooks } from "@commands";
import { ErrorRounded, InfoRounded, WarningRounded } from "@mui/icons-material";
import { Box, Palette, Typography, useTheme } from "@mui/material";
import { Alert } from "@types";
import { memo } from "react";

type AlertSeverity = "warning" | "error" | "info";

const getColor = (palette: Palette, severity: AlertSeverity) => {
    switch (severity) {
        case "error":
            return palette.error.dark;
        case "warning":
            return palette.warning.dark;
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

const AppAlert = memo(function AppAlert() {
    const theme = useTheme();
    const alert: Alert | null = hooks.getAlert("CONFIG_RELOAD")[1];

    const severity = (alert?.severity ?? "info") as AlertSeverity;

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
            </Typography>
        </Box>
    );
});

export default AppAlert;
