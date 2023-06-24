import { Suspense, lazy, memo, useCallback, useState } from "react";
import { AppIcon } from "../AppIcons";
import { useGetTranslation } from "@hooks";
import { SettingsRounded } from "@mui/icons-material";

const SettingsModal = lazy(() => import("./SettingsModal"));

const SettingsIcon = memo(function SettingsIcon() {
    const getTranslation = useGetTranslation();
    const [open, setOpen] = useState(false);

    const onOpen = useCallback(() => {
        setOpen(true);
    }, []);

    const onClose = useCallback(() => {
        setOpen(false);
    }, []);

    return (
        <>
            <Suspense>
                <SettingsModal open={open} onClose={onClose} />
            </Suspense>
            <AppIcon label={getTranslation("SETTINGS")} onClick={onOpen}>
                <SettingsRounded />
            </AppIcon>
        </>
    );
});

export default SettingsIcon;
