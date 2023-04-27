import Icon from "@components/common/Icon";
import NavButton from "./NavButton";
import { BsArrowRepeat } from "react-icons/bs";
import { useTranslation } from "@hooks";
import { useCallback, useEffect, useRef, useState } from "react";
import { commands, hooks } from "@commands";
import { watchImmediate } from "tauri-plugin-fs-watch-api";
import { listen } from "@tauri-apps/api/event";

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

const NavRefreshButton = () => {
    const [watchingFileSystem, setWatchFS] = useState(false);
    const [status, config, err] = hooks.getConfig("CONFIG_RELOAD");
    const guiConfig = hooks.getGuiConfig("GUI_CONFIG_RELOAD")[1];
    const refreshLabel = useTranslation("REFRESH");
    const currentTimeout = useRef<number | null>(null);

    const onRefresh = useCallback(() => {
        const task = async () => {
            setWatchFS(false);
            await commands.refreshLocalDb();
            await commands.refreshRemoteDb();
            await commands.initialSetup();
            setWatchFS(true);
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
                watchImmediate(paths, { recursive: true }, (e) => {
                    if (cancel || !watchingFileSystem || !checkPaths(e.paths)) return;
                    if (currentTimeout.current) {
                        clearTimeout(currentTimeout.current);
                        currentTimeout.current = null;
                    }
                    currentTimeout.current = setTimeout(onRefresh, 500);
                });
            });
        } else if (status === "Error") {
            console.error(err);
        }
        return () => {
            cancel = true;
        };
    }, [onRefresh, err, watchingFileSystem, status, config, guiConfig?.watchFs]);

    return (
        <NavButton onClick={onRefresh} labelPlacement="bottom" ariaLabel={refreshLabel}>
            <Icon iconType={BsArrowRepeat} />
        </NavButton>
    );
};

export default NavRefreshButton;
