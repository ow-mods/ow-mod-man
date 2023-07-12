import { commands, hooks } from "@commands";
import { useGetTranslation } from "@hooks";
import { forwardRef, memo, useCallback, useRef } from "react";
import SettingsForm, { SettingsFormHandle } from "./SettingsForm";
import {
    Box,
    Button,
    CircularProgress,
    Dialog,
    DialogActions,
    DialogContent,
    DialogTitle
} from "@mui/material";
import StyledErrorBoundary from "@components/common/StyledErrorBoundary";

export interface SettingsModalProps {
    open: boolean;
    onClose: () => void;
}

const SettingsModalContent = memo(
    forwardRef(function SettingsModalContent(_, ref) {
        const [configStatus, config] = hooks.getConfig("configReload");
        const [guiConfigStatus, guiConfig] = hooks.getGuiConfig("guiConfigReload");
        const [owmlConfigStatus, owmlConfig] = hooks.getOwmlConfig("owmlConfigReload");

        const status = [configStatus, guiConfigStatus, owmlConfigStatus];

        return status.includes("Loading") &&
            (config === null || guiConfig === null || owmlConfig === null) ? (
            <Box display="flex" alignItems="center" justifyContent="center">
                <CircularProgress color="neutral" />
            </Box>
        ) : (
            <SettingsForm
                key={status.join("-")}
                ref={ref}
                initialConfig={config!}
                initialOwmlConfig={owmlConfig!}
                initialGuiConfig={guiConfig!}
            />
        );
    })
);

const SettingsModal = memo(function SettingsModal({ open, onClose }: SettingsModalProps) {
    const settingsFormRef = useRef<SettingsFormHandle>();
    const getTranslation = useGetTranslation();

    const onSave = useCallback(() => {
        settingsFormRef.current?.save();
        onClose?.();
    }, [onClose]);

    const onCancel = useCallback(() => {
        settingsFormRef.current?.reset();
        onClose?.();
    }, [onClose]);

    const onFix = useCallback(() => {
        commands.getDefaultConfigs().then((defaults) => {
            commands.saveOwmlConfig({ owmlConfig: defaults[2] });
        });
    }, []);

    return (
        <Dialog maxWidth="md" keepMounted fullWidth open={open} onClose={onCancel}>
            <DialogTitle>{getTranslation("SETTINGS")}</DialogTitle>
            <DialogContent dividers>
                <StyledErrorBoundary
                    center
                    errorKey="ERROR_LOADING_OWML_CONFIG"
                    resetEvent="owmlConfigReload"
                    onFix={onFix}
                    fixButtonKey="RESET"
                >
                    <SettingsModalContent ref={settingsFormRef} />
                </StyledErrorBoundary>
            </DialogContent>
            <DialogActions>
                <Button onClick={onCancel}>{getTranslation("CANCEL")}</Button>
                <Button color="primary" variant="contained" onClick={onSave}>
                    {getTranslation("SAVE")}
                </Button>
            </DialogActions>
        </Dialog>
    );
});

export default SettingsModal;
