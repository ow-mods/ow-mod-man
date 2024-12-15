import { useGetTranslation } from "@hooks";
import { ModalProps } from "./About";
import { memo, useCallback } from "react";
import { NorthEastRounded } from "@mui/icons-material";
import { MenuItem, ListItemIcon, ListItemText } from "@mui/material";
import * as dialog from "@tauri-apps/plugin-dialog";
import { commands } from "@commands";
import { simpleOnError } from "../../../../errorHandling";

const Export = memo(function Export({ onClick }: ModalProps) {
    const getTranslation = useGetTranslation();

    const onExport = useCallback(() => {
        dialog
            .save({
                title: getTranslation("EXPORT_MODS"),
                filters: [
                    {
                        name: "JSON File",
                        extensions: ["json"]
                    }
                ]
            })
            .then((path) => {
                if (path) {
                    commands.exportMods({ path }).catch(simpleOnError);
                }
            });
    }, [getTranslation]);

    const handleClick = useCallback(() => {
        onExport();
        onClick?.();
    }, [onClick, onExport]);

    return (
        <>
            <MenuItem onClick={handleClick}>
                <ListItemIcon>
                    <NorthEastRounded fontSize="small" />
                </ListItemIcon>
                <ListItemText>{getTranslation("EXPORT_MODS")}</ListItemText>
            </MenuItem>
        </>
    );
});

export default Export;
