import { memo, useCallback, useState } from "react";
import { AppIcon } from "../AppIcons";
import { useGetTranslation } from "@hooks";
import { SettingsRounded } from "@mui/icons-material";
import SettingsModal from "./SettingsModal";

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
            <SettingsModal open={open} onClose={onClose} />
            <AppIcon label={getTranslation("SETTINGS")} onClick={onOpen}>
                <SettingsRounded />
            </AppIcon>
        </>
    );
});

export default SettingsIcon;
