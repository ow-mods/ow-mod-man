import { useGetTranslation } from "@hooks";
import { ModalProps } from "./About";
import { memo, useCallback, useState } from "react";
import { SouthEastRounded } from "@mui/icons-material";
import {
    MenuItem,
    Checkbox,
    ListItemIcon,
    ListItemText,
    Dialog,
    DialogTitle,
    DialogContent,
    DialogActions,
    Button,
    DialogContentText,
    FormControlLabel,
    useTheme
} from "@mui/material";
import { OpenFileInput } from "@components/common/FileInput";
import { commands } from "@commands";

const Import = memo(function Import({ onClick }: ModalProps) {
    const getTranslation = useGetTranslation();
    const [open, setOpen] = useState(false);
    const theme = useTheme();

    const [filePath, setFilePath] = useState("");
    const [disableMissing, setDisableMissing] = useState(false);

    const handleClick = useCallback(() => {
        setOpen(true);
        onClick?.();
    }, [onClick]);

    const onClose = useCallback(() => {
        setOpen(false);
    }, []);

    const onImport = useCallback(() => {
        commands
            .importMods({ path: filePath, disableMissing })
            .then(() => commands.refreshLocalDb())
            .catch(console.error);
        onClose();
    }, [disableMissing, filePath, onClose]);

    return (
        <>
            <MenuItem onClick={handleClick}>
                <ListItemIcon>
                    <SouthEastRounded fontSize="small" />
                </ListItemIcon>
                <ListItemText>{getTranslation("IMPORT_MODS")}</ListItemText>
            </MenuItem>
            <Dialog open={open} onClose={onClose}>
                <DialogTitle>{getTranslation("IMPORT_MODS")}</DialogTitle>
                <DialogContent dividers>
                    <DialogContentText marginBottom={theme.spacing(1)}>
                        {getTranslation("IMPORT_MODS_EXPLANATION")}
                    </DialogContentText>
                    <OpenFileInput
                        id="import-file-field"
                        label={getTranslation("FILE_PATH")}
                        value={filePath}
                        onChange={setFilePath}
                        dialogOptions={{
                            title: getTranslation("IMPORT_MODS"),
                            filters: [
                                {
                                    name: getTranslation("JSON_FILE"),
                                    extensions: ["json"]
                                }
                            ],
                            directory: false,
                            multiple: false
                        }}
                    />
                    <FormControlLabel
                        checked={disableMissing}
                        onChange={() => setDisableMissing(!disableMissing)}
                        control={<Checkbox />}
                        label={getTranslation("DISABLE_MISSING_MODS")}
                    />
                </DialogContent>
                <DialogActions>
                    <Button color="neutral" onClick={onClose}>
                        {getTranslation("CANCEL")}
                    </Button>
                    <Button onClick={onImport}>{getTranslation("IMPORT")}</Button>
                </DialogActions>
            </Dialog>
        </>
    );
});

export default Import;
