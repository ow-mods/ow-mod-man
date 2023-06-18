import { commands, hooks } from "@commands";
import { useGetTranslation } from "@hooks";
import { listen } from "@tauri-apps/api/event";
import { watchImmediate } from "tauri-plugin-fs-watch-api";
import { memo, useCallback, useEffect, useRef, useState } from "react";
import { CircularProgress } from "@mui/material";
import { RefreshRounded } from "@mui/icons-material";
import { AppIcon } from "./AppIcons";

const checkPaths = (paths: string[]) => {
    for (const path of paths) {
        if (
            path.endsWith("config.json") ||
            path.endsWith("manifest.json") ||
            path.endsWith(".dll") ||
            path.endsWith("OWML.Config.json") ||
            path.endsWith("settings.json") ||
            path.endsWith("gui_settings.json")
        ) {
            return true;
        }
    }
    return false;
};

const ReloadIcon = memo(function ReloadIcon() {
    const getTranslation = useGetTranslation();

    const [isRefreshing, setRefreshing] = useState(false);
    const [watchingFileSystem, setWatchFS] = useState(false);
    const currentTimeout = useRef<number | null>(null);

    const [status, config, err] = hooks.getConfig("CONFIG_RELOAD");
    const guiConfig = hooks.getGuiConfig("GUI_CONFIG_RELOAD")[1];

    const onRefresh = useCallback(() => {
        const task = async () => {
            setRefreshing(true);
            setWatchFS(false);
            await commands.refreshLocalDb();
            await commands.refreshRemoteDb();
            await commands.initialSetup();
            setWatchFS(true);
            setRefreshing(false);
        };
        task();
    }, []);

    useEffect(() => {
        let cancel = false;
        listen("TOGGLE_FS_WATCH", (e) => {
            if (cancel) return;
            const enabled = e.payload as boolean;
            if (!enabled && currentTimeout.current) {
                clearTimeout(currentTimeout.current);
                currentTimeout.current = null;
            }
            setWatchFS(e.payload as boolean);
        });
        return () => {
            cancel = true;
        };
    }, []);

    useEffect(() => {
        let cancel = false;
        if (status === "Done" && (guiConfig?.watchFs ?? false)) {
            commands.getWatcherPaths().then((paths) => {
                watchImmediate(
                    paths,
                    (e) => {
                        if (cancel || !watchingFileSystem || !checkPaths(e.paths)) return;
                        if (currentTimeout.current) {
                            clearTimeout(currentTimeout.current);
                            currentTimeout.current = null;
                        }
                        currentTimeout.current = setTimeout(onRefresh, 500);
                    },
                    { recursive: true }
                );
            });
        } else if (status === "Error") {
            console.error(err);
        }
        return () => {
            cancel = true;
        };
    }, [onRefresh, err, watchingFileSystem, status, config, guiConfig?.watchFs]);

    return (
        <AppIcon disabled={isRefreshing} onClick={onRefresh} label={getTranslation("REFRESH")}>
            {isRefreshing ? <CircularProgress color="secondary" size={24} /> : <RefreshRounded />}
        </AppIcon>
    );
});

export default ReloadIcon;
