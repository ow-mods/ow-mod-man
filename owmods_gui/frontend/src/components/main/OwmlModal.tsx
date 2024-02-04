import { commands } from "@commands";
import { OpenFileInput } from "@components/common/FileInput";
import { useGetTranslation } from "@hooks";
import {
    Box,
    Button,
    Dialog,
    DialogActions,
    DialogContent,
    DialogContentText,
    DialogTitle,
    MenuItem,
    TextField,
    useTheme
} from "@mui/material";
import * as dialog from "@tauri-apps/plugin-dialog";
import { memo, useCallback, useEffect, useState } from "react";
import { listen } from "@events";

type SetupType = "INSTALL_OWML" | "LOCATE_OWML" | "INSTALL_OWML_PRERELEASE";

const OwmlModal = memo(function OwmlModal() {
    const getTranslation = useGetTranslation();
    const theme = useTheme();

    const [open, setOpen] = useState(false);
    const [canCancel, setCanCancel] = useState(false);
    const [installingOwml, setInstallingOwml] = useState(false);

    if (import.meta.env.DEV) {
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        window._DEBUG_OPEN_SETUP = () => setOpen(true);
    }

    const [setupMethod, setSetupMethod] = useState<SetupType>("INSTALL_OWML");
    const [owmlPath, setOwmlPath] = useState("");

    const handleClose = useCallback(() => setOpen(false), []);

    const owmlCheck = useCallback(() => {
        commands.checkOWML().then((valid) => {
            if (!valid) {
                setOpen(true);
                setCanCancel(false);
            }
        });
    }, []);

    const onSubmit = useCallback(() => {
        setInstallingOwml(true);
        if (setupMethod === "INSTALL_OWML" || setupMethod === "INSTALL_OWML_PRERELEASE") {
            commands
                .installOwml({ prerelease: setupMethod === "INSTALL_OWML_PRERELEASE" }, false)
                .then(() => {
                    handleClose();
                })
                .catch((e) => {
                    dialog.message(
                        getTranslation(
                            e === "No prerelease for OWML found"
                                ? "OWML_NO_PRERELEASE"
                                : "OWML_INSTALL_ERROR"
                        ),
                        {
                            type: "error",
                            title: getTranslation("FATAL_ERROR")
                        }
                    );
                })
                .finally(() => setInstallingOwml(false));
        } else {
            commands
                .setOwml({ path: owmlPath }, false)
                .then((valid) => {
                    if (valid) {
                        handleClose();
                    } else {
                        dialog.message(getTranslation("INVALID_OWML"));
                    }
                })
                .catch(dialog.message)
                .finally(() => setInstallingOwml(false));
        }
    }, [getTranslation, handleClose, owmlPath, setupMethod]);

    useEffect(() => owmlCheck(), [owmlCheck]);

    useEffect(() => {
        const unsubscribe = listen("openOwmlSetup", () => {
            setOpen(true);
            setCanCancel(true);
        });
        return unsubscribe;
    }, []);

    useEffect(() => {
        let cancelled = false;
        listen("owmlConfigReload", () => {
            if (cancelled) return;
            owmlCheck();
        });
        return () => {
            cancelled = true;
        };
    }, [owmlCheck]);

    return (
        <Dialog onClose={canCancel ? handleClose : undefined} open={open}>
            <DialogTitle>{getTranslation("SETUP")}</DialogTitle>
            <DialogContent dividers>
                <Box display="flex" flexDirection="column" gap={theme.spacing(1)}>
                    <DialogContentText>{getTranslation("OWML_SETUP_MESSAGE")}</DialogContentText>
                    <TextField
                        variant="outlined"
                        select
                        fullWidth
                        value={setupMethod}
                        onChange={(e) => setSetupMethod(e.target.value as SetupType)}
                    >
                        <MenuItem value="INSTALL_OWML">{getTranslation("INSTALL_OWML")}</MenuItem>
                        <MenuItem value="INSTALL_OWML_PRERELEASE">
                            {getTranslation("INSTALL_OWML_PRERELEASE")}
                        </MenuItem>
                        <MenuItem value="LOCATE_OWML">{getTranslation("LOCATE_OWML")}</MenuItem>
                    </TextField>
                    {setupMethod === "LOCATE_OWML" && (
                        <OpenFileInput
                            id="OWML_PATH"
                            value={owmlPath}
                            label={getTranslation("OWML_PATH")}
                            onChange={setOwmlPath}
                            dialogOptions={{
                                directory: true,
                                title: getTranslation("LOCATE_OWML")
                            }}
                        />
                    )}
                </Box>
            </DialogContent>
            <DialogActions>
                {canCancel && <Button onClick={handleClose}>{getTranslation("CANCEL")}</Button>}
                <Button
                    disabled={installingOwml}
                    color="primary"
                    variant="contained"
                    onClick={onSubmit}
                >
                    {getTranslation("CONTINUE")}
                </Button>
            </DialogActions>
        </Dialog>
    );
});

export default OwmlModal;
