import { useGetTranslation } from "@hooks";
import { ChatBubbleRounded, GitHub, InfoRounded } from "@mui/icons-material";
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
    Box,
    IconButton
} from "@mui/material";
import * as os from "@tauri-apps/plugin-os";
import * as app from "@tauri-apps/plugin-app";
import * as shell from "@tauri-apps/plugin-shell";
import { memo, useCallback, useEffect, useState } from "react";
import logo from "@assets/images/logo.png?w=256&h=256&format=webp&imagetools";
import ODTooltip from "@components/common/ODTooltip";
import { commands } from "@commands";

export interface ModalProps {
    onClick?: () => void;
}

const commitHash = import.meta.env.VITE_COMMIT;
const versionSuffix = import.meta.env.VITE_VERSION_SUFFIX;

const About = memo(function About({ onClick }: ModalProps) {
    const getTranslation = useGetTranslation();

    const [open, setOpen] = useState(false);

    const [appVersion, setVersion] = useState("");
    const [appPlatform, setPlatform] = useState("");
    const [archRaw, setArch] = useState("");

    useEffect(() => {
        const onErr = (e: string) => {
            commands.logError({ err: e.toString() });
        };

        app.getVersion()
            .then(setVersion)
            .catch((e) => {
                onErr(e);
                setVersion("Error");
            });
        os.platform()
            .then(setPlatform)
            .catch((e) => {
                onErr(e);
                setPlatform("Error");
            });
        os.arch()
            .then(setArch)
            .catch((e) => {
                onErr(e);
                setArch("Error");
            });
    }, []);

    const handleClick = useCallback(() => {
        setOpen(true);
        onClick?.();
    }, [onClick]);

    const onClose = useCallback(() => {
        setOpen(false);
    }, []);

    const onGithub = useCallback(() => {
        shell.open("https://github.com/ow-mods/ow-mod-man/");
    }, []);

    const onDiscord = useCallback(() => {
        shell.open("https://discord.gg/wusTQYbYTc");
    }, []);

    return (
        <>
            <MenuItem onClick={handleClick}>
                <ListItemIcon>
                    <InfoRounded fontSize="small" />
                </ListItemIcon>
                <ListItemText>{getTranslation("ABOUT")}</ListItemText>
            </MenuItem>
            <Dialog fullWidth maxWidth="sm" open={open} onClose={onClose}>
                <DialogTitle>{getTranslation("ABOUT")}</DialogTitle>
                <DialogContent dividers>
                    <Box width="100%" display="flex" justifyContent="center">
                        <img width="256" height="256" src={logo} />
                    </Box>
                    <DialogContentText align="center">
                        <h1 style={{ margin: 0 }}>{getTranslation("APP_TITLE")}</h1>
                        <Box justifyContent="center" display="flex">
                            <ODTooltip title={getTranslation("GITHUB")}>
                                <IconButton onClick={onGithub}>
                                    <GitHub />
                                </IconButton>
                            </ODTooltip>
                            <ODTooltip title={getTranslation("DISCORD")}>
                                <IconButton onClick={onDiscord}>
                                    <ChatBubbleRounded />
                                </IconButton>
                            </ODTooltip>
                        </Box>
                        {getTranslation("APP_VERSION", {
                            version: `${appVersion}${versionSuffix ?? ""}`
                        })}
                        <br />
                        {getTranslation("PLATFORM", { platform: appPlatform })}
                        <br />
                        {getTranslation("ARCHITECTURE", { arch: archRaw })}
                        <br />
                        {commitHash ?? ""}
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
