import { useGetTranslation } from "@hooks";
import { ModalProps } from "./About";
import { memo, useCallback, useEffect, useMemo, useState } from "react";
import { InstallDesktopRounded, WarningRounded } from "@mui/icons-material";
import {
    MenuItem,
    ListItemIcon,
    ListItemText,
    Dialog,
    DialogTitle,
    DialogContent,
    DialogActions,
    Button,
    Select,
    InputLabel,
    FormControl,
    TextField,
    Box,
    useTheme,
    FormControlLabel,
    Checkbox,
    DialogContentText
} from "@mui/material";
import { ProtocolInstallType } from "@types";
import { commands } from "@commands";
import { getCurrent } from "@tauri-apps/api/window";
import { OpenFileInput } from "@components/common/FileInput";
import { listen } from "@events";

type SourceType = "UNIQUE_NAME" | "URL" | "ZIP";

const getSourceTypeFromProtocol = (installType: ProtocolInstallType): SourceType | null => {
    switch (installType) {
        case "installMod":
            return "UNIQUE_NAME";
        case "installURL":
            return "URL";
        case "installPreRelease":
            return "UNIQUE_NAME";
        case "installZip":
            return "ZIP";
        default:
            return null;
    }
};

const InstallFrom = memo(function InstallFrom({ onClick }: ModalProps) {
    const getTranslation = useGetTranslation();
    const [open, setOpen] = useState(false);
    const theme = useTheme();

    const [source, setSource] = useState<SourceType>("UNIQUE_NAME");
    const [target, setTarget] = useState<string>("");
    const [prerelease, setPrerelease] = useState<boolean>(false);

    const lblMap: Record<SourceType, string> = useMemo(
        () => ({
            UNIQUE_NAME: getTranslation("UNIQUE_NAME"),
            URL: getTranslation("URL"),
            ZIP: getTranslation("ZIP")
        }),
        [getTranslation]
    );

    useEffect(() => {
        let cancel = false;
        listen("protocolInvoke", (protocolPayload) => {
            if (cancel) return;
            commands.checkOWML().then((valid) => {
                if (valid) {
                    const sourceType = getSourceTypeFromProtocol(protocolPayload.installType);
                    if (sourceType !== null) {
                        setSource(sourceType);
                        setTarget(protocolPayload.payload);
                        if (
                            protocolPayload.installType === "installPreRelease" ||
                            protocolPayload.installType === "installMod"
                        ) {
                            setPrerelease(protocolPayload.installType === "installPreRelease");
                        }
                        setOpen(true);
                        getCurrent().setFocus().catch(commands.logError);
                    }
                }
            });
        })
            .then(() => commands.popProtocolURL())
            .catch(commands.logError);
        return () => {
            cancel = true;
        };
    }, []);

    const handleClick = useCallback(() => {
        setOpen(true);
        onClick?.();
    }, [onClick]);

    const onClose = useCallback(() => {
        setOpen(false);
    }, []);

    const onInstall = useCallback(() => {
        switch (source) {
            case "UNIQUE_NAME":
                commands
                    .installMod({ uniqueName: target, prerelease })
                    .then(() => commands.refreshLocalDb())
                    .catch(commands.logError);
                break;
            case "URL":
                commands
                    .installUrl({ url: target })
                    .then(() => commands.refreshLocalDb())
                    .catch(commands.logError);
                break;
            case "ZIP":
                commands
                    .installZip({ path: target })
                    .then(() => commands.refreshLocalDb())
                    .catch(commands.logError);
                break;
        }
        onClose();
    }, [onClose, prerelease, source, target]);

    return (
        <>
            <MenuItem onClick={handleClick}>
                <ListItemIcon>
                    <InstallDesktopRounded fontSize="small" />
                </ListItemIcon>
                <ListItemText>{getTranslation("INSTALL_FROM")}</ListItemText>
            </MenuItem>
            <Dialog fullWidth open={open} onClose={onClose}>
                <DialogTitle>{getTranslation("INSTALL_FROM")}</DialogTitle>
                <DialogContent dividers>
                    <Box display="flex" flexDirection="column" gap={theme.spacing(2)}>
                        <FormControl>
                            <InputLabel id="install-from-source-selector">
                                {getTranslation("INSTALL_FROM")}
                            </InputLabel>
                            <Select
                                value={source}
                                onChange={(e) => {
                                    setTarget("");
                                    setSource(e.target.value as SourceType);
                                }}
                                label={getTranslation("INSTALL_FROM")}
                                fullWidth
                            >
                                {Object.entries(lblMap).map(([key, val]) => (
                                    <MenuItem key={key} value={key}>
                                        {val}
                                    </MenuItem>
                                ))}
                            </Select>
                        </FormControl>
                        {source === "ZIP" ? (
                            <OpenFileInput
                                id={source}
                                value={target}
                                label={getTranslation("FILE_PATH")}
                                onChange={setTarget}
                                dialogOptions={{
                                    title: getTranslation("INSTALL_FROM"),
                                    filters: [
                                        {
                                            name: lblMap[source],
                                            extensions: ["zip"]
                                        }
                                    ],
                                    directory: false,
                                    multiple: false
                                }}
                            />
                        ) : (
                            <TextField
                                id="install-from-target-text"
                                variant="outlined"
                                value={target}
                                label={lblMap[source]}
                                onChange={(e) => setTarget(e.target.value)}
                            />
                        )}
                        {source === "UNIQUE_NAME" ? (
                            <FormControlLabel
                                checked={prerelease}
                                onChange={() => setPrerelease(!prerelease)}
                                control={<Checkbox />}
                                label={getTranslation("USE_PRERELEASE_CHECKBOX")}
                            />
                        ) : (
                            <Box display="flex" alignItems="center" gap={theme.spacing(2)}>
                                <WarningRounded color="warning" fontSize="large" />
                                <DialogContentText>
                                    {getTranslation("INSTALL_WARNING")}
                                </DialogContentText>
                            </Box>
                        )}
                    </Box>
                </DialogContent>
                <DialogActions>
                    <Button onClick={onClose}>{getTranslation("CANCEL")}</Button>
                    <Button variant="contained" color="primary" onClick={onInstall}>
                        {getTranslation("INSTALL")}
                    </Button>
                </DialogActions>
            </Dialog>
        </>
    );
});

export default InstallFrom;
