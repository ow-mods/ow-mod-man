import { OpenModValidationModalPayload } from "@components/modals/ModValidationModal";
import { FailedMod, ModValidationError } from "@types";
import LocalModRow from "./LocalModRow";
import { confirm } from "@tauri-apps/api/dialog";
import { useCallback } from "react";
import { commands } from "@commands";
import { useGetTranslation } from "@hooks";

export interface FailedModRowProps {
    mod: FailedMod;
    onValidationClick?: (p: OpenModValidationModalPayload) => void;
}

const FailedModRow = ({ mod, onValidationClick }: FailedModRowProps) => {
    const getTranslation = useGetTranslation();

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
        confirm(
            getTranslation("UNINSTALL_CONFIRM", {
                name: mod.displayPath
            }),
            getTranslation("CONFIRM")
        ).then((answer) => {
            if (answer) {
                commands
                    .uninstallBrokenMod({ modPath: mod.modPath })
                    .then(() => commands.refreshLocalDb());
            }
        });
    }, [getTranslation, mod.displayPath, mod.modPath]);

    const onOpen = useCallback(() => {
        commands.openModFolder({ uniqueName: mod.modPath });
    }, [mod.modPath]);

    return (
        <LocalModRow
            uniqueName={mod.modPath}
            name={mod.displayPath}
            subtitle={getTranslation("CANT_LOAD")}
            errors={[mod.error]}
            showValidation
            onValidationClick={onValidationClicked}
            onUninstall={onUninstall}
            onOpen={onOpen}
        />
    );
};

export default FailedModRow;
