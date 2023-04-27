import { OpenModValidationModalPayload } from "@components/modals/ModValidationModal";
import { FailedMod, ModValidationError } from "@types";
import LocalModRow from "./LocalModRow";
import { confirm } from "@tauri-apps/api/dialog";
import { useCallback } from "react";
import { commands } from "@commands";
import { useTranslation, useTranslations } from "@hooks";

export interface FailedModRowProps {
    mod: FailedMod;
    onValidationClick?: (p: OpenModValidationModalPayload) => void;
}

const FailedModRow = ({ mod, onValidationClick }: FailedModRowProps) => {
    const [confirmText, cantLoad] = useTranslations(["CONFIRM", "CANT_LOAD"]);

    const uninstallConfirmText = useTranslation("UNINSTALL_CONFIRM", {
        name: mod.displayPath
    });

    const onValidationClicked = useCallback(
        (errs: ModValidationError[]) => {
            onValidationClick?.({
                uniqueName: mod.modPath,
                modName: mod.displayPath,
                errors: errs
            });
        },
        [mod.modPath, mod.displayPath, onValidationClick]
    );

    const onUninstall = useCallback(() => {
        confirm(uninstallConfirmText, confirmText).then((answer) => {
            if (answer) {
                commands
                    .uninstallBrokenMod({ modPath: mod.modPath })
                    .then(() => commands.refreshLocalDb());
            }
        });
    }, [mod.modPath, confirmText, uninstallConfirmText]);

    const onOpen = useCallback(() => {
        commands.openModFolder({ uniqueName: mod.modPath });
    }, [mod.modPath]);

    return (
        <LocalModRow
            uniqueName={mod.modPath}
            name={mod.displayPath}
            subtitle={cantLoad}
            errors={[mod.error]}
            showValidation
            onValidationClick={onValidationClicked}
            onUninstall={onUninstall}
            onOpen={onOpen}
        />
    );
};

export default FailedModRow;
