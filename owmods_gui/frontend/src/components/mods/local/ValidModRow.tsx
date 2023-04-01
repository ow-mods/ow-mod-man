import { commands } from "@commands";
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

const ValidModRow = memo((props: LocalModRowProps) => {
    const confirmText = useTranslation("CONFIRM");

    const uninstallConfirmText = useTranslation("UNINSTALL_CONFIRM", {
        name: props.mod.manifest.name
    });

    const subtitle = useTranslation("BY", {
        author: props.mod.manifest.author,
        version: props.mod.manifest.version
    });

    const onValidationClicked = useCallback(
        (errs: ModValidationError[]) => {
            props.onValidationClick?.({
                uniqueName: props.mod.manifest.uniqueName,
                modName: props.mod.manifest.name,
                errors: errs
            });
        },
        [props.mod.manifest.name, props.mod.errors]
    );

    const onToggle = useCallback(
        (newVal: boolean) => {
            commands
                .toggleMod({
                    uniqueName: props.mod.manifest.uniqueName,
                    enabled: newVal
                })
                .then(() => commands.refreshLocalDb());
        },
        [props.mod.manifest.uniqueName]
    );

    const onOpen = useCallback(() => {
        commands.openModFolder({ uniqueName: props.mod.manifest.uniqueName });
    }, [props.mod.manifest.uniqueName]);

    const onUninstall = useCallback(() => {
        confirm(uninstallConfirmText, confirmText).then((answer) => {
            if (answer) {
                commands
                    .uninstallMod({ uniqueName: props.mod.manifest.uniqueName })
                    .then(() => commands.refreshLocalDb());
            }
        });
    }, [props.mod.manifest.uniqueName, props.mod.manifest.name]);

    return (
        <LocalModRow
            name={props.mod.manifest.name}
            showValidation={props.mod.enabled}
            enabled={props.mod.enabled}
            errors={props.mod.errors}
            subtitle={subtitle}
            onOpen={onOpen}
            onToggle={onToggle}
            onUninstall={onUninstall}
            onValidationClick={onValidationClicked}
        />
    );
});

export default ValidModRow;
