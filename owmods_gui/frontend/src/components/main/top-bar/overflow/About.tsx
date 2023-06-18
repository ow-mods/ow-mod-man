import { useGetTranslation } from "@hooks";
import { InfoRounded } from "@mui/icons-material";
import {
    MenuItem,
    ListItemIcon,
    ListItemText,
    Dialog,
    DialogTitle,
    DialogContent,
    DialogActions,
    Button,
    DialogContentText,
    Box
} from "@mui/material";
import { app, os } from "@tauri-apps/api";
import { memo, useCallback, useEffect, useState } from "react";
import logo from "@assets/images/logo.png?w=256&h=256&format=webp&imagetools";

export interface ModalProps {
    onClick?: () => void;
}

const About = memo(function About({ onClick }: ModalProps) {
    const getTranslation = useGetTranslation();

    const [open, setOpen] = useState(false);

    const [appVersion, setVersion] = useState("");
    const [appPlatform, setPlatform] = useState("");
    const [archRaw, setArch] = useState("");

    useEffect(() => {
        app.getVersion().then(setVersion);
        os.platform().then(setPlatform);
        os.arch().then(setArch);
    }, []);

    const handleClick = useCallback(() => {
        setOpen(true);
        onClick?.();
    }, [onClick]);

    const onClose = useCallback(() => {
        setOpen(false);
    }, []);

    return (
        <>
            <MenuItem onClick={handleClick}>
                <ListItemIcon>
                    <InfoRounded fontSize="small" />
                </ListItemIcon>
                <ListItemText>{getTranslation("ABOUT")}</ListItemText>
            </MenuItem>
            <Dialog maxWidth="md" open={open} onClose={onClose}>
                <DialogTitle>{getTranslation("ABOUT")}</DialogTitle>
                <DialogContent dividers>
                    <Box width="100%" display="flex" justifyContent="center">
                        <img width="256" height="256" src={logo} />
                    </Box>
                    <DialogContentText align="center">
                        <h1 style={{ margin: 0 }}>{getTranslation("APP_TITLE")}</h1>
                        <br />
                        {getTranslation("APP_VERSION", { version: appVersion })}
                        <br />
                        {getTranslation("PLATFORM", { platform: appPlatform })}
                        <br />
                        {getTranslation("ARCHITECTURE", { arch: archRaw })}
                        <br />
                    </DialogContentText>
                </DialogContent>
                <DialogActions>
                    <Button onClick={onClose}>{getTranslation("DISMISS")}</Button>
                </DialogActions>
            </Dialog>
        </>
    );
});

export default About;
