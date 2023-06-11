import { DescriptionRounded, FolderRounded, DeleteRounded } from "@mui/icons-material";
import { Checkbox, useTheme } from "@mui/material";
import { memo, useRef } from "react";
import ModActionIcon from "../ModActionIcon";
import ModActionOverflow, { ModActionOverflowItem } from "../ModActionOverflow";
import { useGetTranslation } from "@hooks";

export interface LocalModActionsProps {
    uniqueName: string;
    enabled: boolean;
    onToggle: (newVal: boolean) => void;
    onReadme: () => void;
    onFolder: () => void;
    onUninstall: () => void;
}

const LocalModActions = memo(function LocalModTools(props: LocalModActionsProps) {
    const theme = useTheme();
    const getTranslation = useGetTranslation();
    const overflowRef = useRef<{ onClose: () => void }>();

    return (
        <>
            <Checkbox
                sx={{ color: theme.palette.grey[200] }}
                color="default"
                onChange={(e) => props.onToggle(e.target.checked)}
                checked={props.enabled}
            />
            <ModActionIcon
                onClick={props.onReadme}
                label={getTranslation("OPEN_WEBSITE")}
                icon={<DescriptionRounded />}
            />
            <ModActionOverflow id={`local-${props.uniqueName}`} ref={overflowRef}>
                <ModActionOverflowItem
                    label={getTranslation("SHOW_FOLDER")}
                    icon={<FolderRounded />}
                    onClick={props.onFolder}
                    onClose={overflowRef.current?.onClose}
                />
                <ModActionOverflowItem
                    label={getTranslation("UNINSTALL")}
                    icon={<DeleteRounded />}
                    onClick={props.onUninstall}
                    onClose={overflowRef.current?.onClose}
                />
            </ModActionOverflow>
        </>
    );
});

export default LocalModActions;
