import {
    ConstructionRounded,
    DeleteRounded,
    DescriptionRounded,
    DownloadRounded,
    FolderRounded,
    GitHub
} from "@mui/icons-material";
import { Checkbox, useTheme } from "@mui/material";
import { memo, useRef } from "react";
import ModActionIcon from "../ModActionIcon";
import ModActionOverflow, { ModActionOverflowItem } from "../ModActionOverflow";
import { useGetTranslation } from "@hooks";
import { hooks } from "@commands";
import LocalModDonateIcon from "./LocalModDonateIcon";

export interface LocalModActionsProps {
    uniqueName: string;
    enabled: boolean;
    isErr: boolean;
    hasRemote: boolean;
    canFix: boolean;
    donateLinks?: string[];
    onToggle: (newVal: boolean) => void;
    onReadme: () => void;
    onFolder: () => void;
    onFix: () => void;
    onGithub: () => void;
    onUninstall: () => void;
    onReinstall: () => void;
}

const LocalModActions = memo(function LocalModTools(props: LocalModActionsProps) {
    const theme = useTheme();
    const getTranslation = useGetTranslation();
    const guiConfig = hooks.getGuiConfig("guiConfigReload")[1];
    const overflowRef = useRef<{ onClose: () => void }>({
        onClose: () => {}
    });

    const isBusy = hooks.getModBusy("modBusy", { uniqueName: props.uniqueName })[1];
    // Disable the fix button if ANY mods are busy, this is to stop the user from clicking fix when a dep is installing
    const isAnyBusy = (hooks.getBusyMods("modBusy")[1] ?? []).length !== 0;

    return (
        <>
            {!guiConfig?.hideDonate && props.donateLinks && props.donateLinks.length !== 0 && (
                <LocalModDonateIcon uniqueName={props.uniqueName} links={props.donateLinks} />
            )}
            <Checkbox
                sx={{ color: theme.palette.grey[200] }}
                color={props.isErr || isBusy ? "primary" : "default"}
                onChange={(e) => props.onToggle(e.target.checked)}
                disabled={props.isErr || (isBusy ?? false)}
                checked={props.enabled}
            />
            {props.canFix ? (
                <ModActionIcon
                    icon={<ConstructionRounded />}
                    onClick={props.onFix}
                    label={getTranslation("FIX")}
                    disabled={isAnyBusy ?? false}
                />
            ) : (
                <ModActionIcon
                    onClick={props.onReadme}
                    label={getTranslation("OPEN_README")}
                    disabled={!props.hasRemote}
                    icon={<DescriptionRounded />}
                />
            )}
            <ModActionOverflow tabId="local" uniqueName={props.uniqueName} ref={overflowRef}>
                {props.canFix && (
                    <ModActionOverflowItem
                        label={getTranslation("OPEN_README")}
                        disabled={!props.hasRemote}
                        icon={<DescriptionRounded />}
                        onClick={props.onReadme}
                        onClose={overflowRef.current?.onClose}
                    />
                )}
                <ModActionOverflowItem
                    label={getTranslation("SHOW_FOLDER")}
                    icon={<FolderRounded />}
                    onClick={props.onFolder}
                    onClose={overflowRef.current?.onClose}
                />
                {props.hasRemote && (
                    <>
                        <ModActionOverflowItem
                            label={getTranslation("OPEN_GITHUB")}
                            icon={<GitHub />}
                            onClick={props.onGithub}
                            onClose={overflowRef.current?.onClose}
                        />
                        <ModActionOverflowItem
                            label={getTranslation("REINSTALL")}
                            icon={<DownloadRounded />}
                            disabled={isBusy ?? true}
                            onClick={props.onReinstall}
                            onClose={overflowRef.current?.onClose}
                        />
                    </>
                )}
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
