import { commands, hooks } from "@commands";
import { OpenModValidationModalPayload } from "@components/modals/ModValidationModal";
import { useTranslation } from "@hooks";
import { confirm } from "@tauri-apps/api/dialog";
import { LocalMod, ModValidationError } from "@types";
import { memo, useCallback } from "react";
import LocalModRow from "./LocalModRow";

interface LocalModRowProps {
    mod: LocalMod;
    onValidationClick?: (p: OpenModValidationModalPayload) => void;
}

const ValidModRow = memo(({ mod, onValidationClick }: LocalModRowProps) => {
    const confirmText = useTranslation("CONFIRM");

    const remoteMod = hooks.getRemoteMod("REMOTE-REFRESH", {
        uniqueName: mod.manifest.uniqueName
    })[1];

    const uninstallConfirmText = useTranslation("UNINSTALL_CONFIRM", {
        name: mod.manifest.name
    });

    const subtitle = useTranslation("BY", {
        author: mod.manifest.author,
        version: mod.manifest.version
    });

    const onValidationClicked = useCallback(
        (errs: ModValidationError[]) => {
            onValidationClick?.({
                uniqueName: mod.manifest.uniqueName,
                modName: mod.manifest.name,
                errors: errs
            });
        },
        [mod.manifest.uniqueName, mod.manifest.name, onValidationClick]
    );

    const onToggle = useCallback(
        (newVal: boolean) => {
            commands
                .toggleMod({
                    uniqueName: mod.manifest.uniqueName,
                    enabled: newVal
                })
                .then(() => commands.refreshLocalDb());
        },
        [mod.manifest.uniqueName]
    );

    const onOpen = useCallback(() => {
        commands.openModFolder({ uniqueName: mod.manifest.uniqueName });
    }, [mod.manifest.uniqueName]);

    const onUninstall = useCallback(() => {
        confirm(uninstallConfirmText, confirmText).then((answer) => {
            if (answer) {
                commands
                    .uninstallMod({ uniqueName: mod.manifest.uniqueName })
                    .then(() => commands.refreshLocalDb());
            }
        });
    }, [mod.manifest.uniqueName, confirmText, uninstallConfirmText]);

    return (
        <LocalModRow
            uniqueName={mod.manifest.uniqueName}
            name={mod.manifest.name}
            showValidation={mod.enabled}
            enabled={mod.enabled}
            description={remoteMod?.description}
            readme
            errors={mod.errors}
            subtitle={subtitle}
            onOpen={onOpen}
            onToggle={onToggle}
            onUninstall={onUninstall}
            onValidationClick={onValidationClicked}
        />
    );
});

export default ValidModRow;
