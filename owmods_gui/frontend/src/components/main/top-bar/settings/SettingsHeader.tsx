import { useGetTranslation } from "@hooks";
import { SettingsBackupRestoreRounded } from "@mui/icons-material";
import { Box, Button, Typography, useTheme } from "@mui/material";
import { memo } from "react";

export interface SettingsHeaderProps {
    text: string;
    onReset?: () => void;
}

const ResetButton = memo(function ResetButton(props: { onClick?: () => void }) {
    const getTranslation = useGetTranslation();

    return (
        <Button
            color="secondary"
            onClick={props.onClick}
            startIcon={<SettingsBackupRestoreRounded />}
        >
            {getTranslation("RESET")}
        </Button>
    );
});

const SettingsHeader = memo(function SettingsHeader(props: SettingsHeaderProps) {
    const theme = useTheme();

    return (
        <Box display="flex" gap={theme.spacing(2)}>
            <Typography flexGrow={1} variant="h5" marginTop={theme.spacing(1)}>
                {props.text}
            </Typography>
            <ResetButton onClick={props.onReset} />
        </Box>
    );
});

export default SettingsHeader;
