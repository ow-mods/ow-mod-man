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

const FailedModRow = (props: FailedModRowProps) => {
    const [confirmText, cantLoad] = useTranslations(["CONFIRM", "CANT_LOAD"]);

    const uninstallConfirmText = useTranslation("UNINSTALL_CONFIRM", {
        name: props.mod.displayPath
    });

    const onValidationClicked = useCallback(
        (errs: ModValidationError[]) => {
            props.onValidationClick?.({
                uniqueName: props.mod.modPath,
                modName: props.mod.displayPath,
                errors: errs
            });
        },
        [props.mod.modPath, props.mod.displayPath, props.mod.error]
    );

    const onUninstall = useCallback(() => {
        confirm(uninstallConfirmText, confirmText).then((answer) => {
            if (answer) {
                commands
                    .uninstallBrokenMod({ modPath: props.mod.modPath })
                    .then(() => commands.refreshLocalDb());
            }
        });
    }, [props.mod.modPath, props.mod.displayPath]);

    const onOpen = useCallback(() => {
        commands.openModFolder({ uniqueName: props.mod.modPath });
    }, [props.mod.modPath]);

    return (
        <LocalModRow
            uniqueName={props.mod.modPath}
            name={props.mod.displayPath}
            subtitle={cantLoad}
            errors={[props.mod.error]}
            showValidation
            onValidationClick={onValidationClicked}
            onUninstall={onUninstall}
            onOpen={onOpen}
        />
    );
};

export default FailedModRow;
