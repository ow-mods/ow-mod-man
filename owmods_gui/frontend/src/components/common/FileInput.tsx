import { useGetTranslation } from "@hooks";
import { dialog } from "@tauri-apps/api";
import { FolderRounded } from "@mui/icons-material";
import { Box, TextField, Button, useTheme } from "@mui/material";

export interface FileInputProps<T> {
    dialogOptions: T;
    id: string;
    label?: string;
    value?: string;
    onChange?: (path: string) => void;
}

const FileInput = <T,>(openFunc: (options?: T) => Promise<string | string[] | null>) =>
    function FileInput(props: FileInputProps<T>) {
        const theme = useTheme();
        const getTranslation = useGetTranslation();

        const onBrowse = () => {
            openFunc(props.dialogOptions).then((path) => {
                if (path !== null) {
                    props.onChange?.(path as string);
                }
            });
        };

        return (
            <Box display="flex" gap={theme.spacing(2)}>
                <TextField
                    variant="outlined"
                    value={props.value}
                    onChange={(e) => props.onChange?.(e.target.value)}
                    id={props.id}
                    label={props.label}
                    sx={{ flexGrow: 1 }}
                />
                <Button onClick={onBrowse} startIcon={<FolderRounded />}>
                    {getTranslation("BROWSE")}
                </Button>
            </Box>
        );
    };

export const OpenFileInput = FileInput(dialog.open);
export const SaveFileInput = FileInput(dialog.save);
