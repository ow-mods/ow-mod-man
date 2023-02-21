import Icon from "@components/Icon";
import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { useTauri, useTranslation, useTranslations } from "@hooks";
import { invoke } from "@tauri-apps/api";
import { confirm } from "@tauri-apps/api/dialog";
import { LocalMod } from "@types";
import { memo, useCallback } from "react";
import { FaFolder, FaTrash } from "react-icons/fa";

interface LocalModRowProps {
    uniqueName: string;
}

const LocalModRow = memo((props: LocalModRowProps) => {
    const [status, mod, err] = useTauri<LocalMod>("LOCAL-REFRESH", "get_local_mod", {
        uniqueName: props.uniqueName
    });

    const [showFolderTooltip, uninstallTooltip, confirmText] = useTranslations([
        "SHOW_FOLDER",
        "UNINSTALL",
        "CONFIRM"
    ]);
    const uninstallConfirmText = useTranslation("UNINSTALL_CONFIRM", {
        name: mod?.manifest.name ?? "null"
    });

    const subtitle = useTranslation("BY", { author: mod?.manifest.author ?? "" });

    const onToggle = useCallback(
        (newVal: boolean) => {
            invoke("toggle_mod", {
                uniqueName: props.uniqueName,
                enabled: newVal
            }).then(() => invoke("refresh_local_db"));
        },
        [props.uniqueName]
    );

    const onOpen = useCallback(() => {
        invoke("open_mod_folder", { uniqueName: props.uniqueName });
    }, [props.uniqueName]);

    const onUninstall = useCallback(() => {
        confirm(uninstallConfirmText, confirmText).then((answer) => {
            if (answer) {
                invoke("uninstall_mod", { uniqueName: props.uniqueName }).then(() =>
                    invoke("refresh_local_db")
                );
            }
        });
    }, [props.uniqueName, mod?.manifest.name]);

    if (status === "Loading" && mod === null) {
        return <div className="mod-row local center-loading" aria-busy></div>;
    } else if (status === "Error") {
        return <div className="mod-row local center-loading">{err!.toString()}</div>;
    } else {
        const localMod = mod!;
        return (
            <div className="mod-row local">
                <ModHeader subtitle={subtitle} {...localMod.manifest}>
                    <ModActionButton onClick={onOpen} ariaLabel={showFolderTooltip}>
                        <Icon iconType={FaFolder} />
                    </ModActionButton>
                    <ModActionButton onClick={onUninstall} ariaLabel={uninstallTooltip}>
                        <Icon iconType={FaTrash} />
                    </ModActionButton>
                    <input
                        className="mod-toggle"
                        type="checkbox"
                        aria-label="enabled"
                        role="switch"
                        onChange={(e) => onToggle(e.target.checked)}
                        checked={localMod.enabled}
                    />
                </ModHeader>
            </div>
        );
    }
});

export default LocalModRow;
