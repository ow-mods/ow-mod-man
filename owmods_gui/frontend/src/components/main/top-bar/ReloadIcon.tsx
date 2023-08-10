import { commands } from "@commands";
import { useGetTranslation } from "@hooks";
import { memo, useCallback, useEffect, useRef, useState } from "react";
import { CircularProgress } from "@mui/material";
import { RefreshRounded } from "@mui/icons-material";
import { AppIcon } from "./AppIcons";
import { useErrorBoundary } from "react-error-boundary";
import { listen } from "@events";

type Timeout = ReturnType<typeof setTimeout>;

const ReloadIcon = memo(function ReloadIcon() {
    const getTranslation = useGetTranslation();

    const [isRefreshing, setRefreshing] = useState(false);
    const currentTimeout = useRef<Timeout | null>(null);

    const errorBound = useErrorBoundary();

    const onRefresh = useCallback(
        (refreshLocal: boolean, refreshRemote: boolean, refreshConfigs: boolean) => {
            const task = async () => {
                setRefreshing(true);
                if (refreshLocal) {
                    await commands.refreshLocalDb();
                }
                if (refreshConfigs) {
                    await commands.initialSetup({}, false).catch((e) => errorBound.showBoundary(e));
                }
                if (refreshRemote) {
                    await commands.refreshRemoteDb({}, false);
                }
            };
            task().finally(() => {
                setRefreshing(false);
            });
        },
        [errorBound]
    );

    const onRefreshButton = useCallback(() => {
        onRefresh(true, true, true);
    }, [onRefresh]);

    useEffect(() => {
        const unsubscribe = listen("requestReload", (reloadType) => {
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
        return unsubscribe;
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
