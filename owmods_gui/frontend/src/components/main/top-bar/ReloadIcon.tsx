import { commands } from "@commands";
import { useGetTranslation } from "@hooks";
import { memo, useCallback, useEffect, useRef, useState } from "react";
import { CircularProgress } from "@mui/material";
import { RefreshRounded } from "@mui/icons-material";
import { AppIcon } from "./AppIcons";
import { listen } from "@tauri-apps/api/event";

const ReloadIcon = memo(function ReloadIcon() {
    const getTranslation = useGetTranslation();

    const [isRefreshing, setRefreshing] = useState(false);
    const currentTimeout = useRef<number | null>(null);

    const onRefresh = useCallback(
        (refreshLocal: boolean, refreshRemote: boolean, refreshConfigs: boolean) => {
            const task = async () => {
                setRefreshing(true);
                if (refreshLocal) {
                    await commands.refreshLocalDb();
                }
                if (refreshConfigs) {
                    await commands.initialSetup();
                }
                if (refreshRemote) {
                    await commands.refreshRemoteDb();
                }
            };
            task().finally(() => {
                setRefreshing(false);
            });
        },
        []
    );

    const onRefreshButton = useCallback(() => {
        onRefresh(true, true, true);
    }, [onRefresh]);

    useEffect(() => {
        let cancel = false;
        listen("REQUEST-RELOAD", (e) => {
            if (cancel) return;
            const reloadType = e.payload as string;
            if (currentTimeout.current !== null) {
                clearTimeout(currentTimeout.current);
            }
            currentTimeout.current = setTimeout(() => {
                onRefresh(
                    reloadType === "LOCAL",
                    false,
                    reloadType === "GUI" || reloadType === "CONFIG"
                );
            }, 500);
        });
        return () => {
            cancel = true;
        };
    }, [onRefresh]);

    return (
        <AppIcon
            disabled={isRefreshing}
            onClick={onRefreshButton}
            label={getTranslation("REFRESH")}
        >
            {isRefreshing ? <CircularProgress color="secondary" size={24} /> : <RefreshRounded />}
        </AppIcon>
    );
});
export default ReloadIcon;
