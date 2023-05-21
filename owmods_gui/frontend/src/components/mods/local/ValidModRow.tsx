import { commands, hooks } from "@commands";
import { OpenModValidationModalPayload } from "@components/modals/ModValidationModal";
import { useGetTranslation } from "@hooks";
import { confirm } from "@tauri-apps/api/dialog";
import { LocalMod, ModValidationError } from "@types";
import { memo, useCallback } from "react";
import LocalModRow from "./LocalModRow";
import { dialog } from "@tauri-apps/api";

interface LocalModRowProps {
    mod: LocalMod;
    onValidationClick?: (p: OpenModValidationModalPayload) => void;
}

const ValidModRow = memo(function ValidModRow({ mod, onValidationClick }: LocalModRowProps) {
    const getTranslation = useGetTranslation();

    const remoteMod = hooks.getRemoteMod("REMOTE-REFRESH", {
        uniqueName: mod.manifest.uniqueName
    })[1];

    const autoEnableDeps = hooks.getGuiConfig("GUI_CONFIG_RELOAD")[1]?.autoEnableDeps ?? false;

    const uninstallConfirmText = getTranslation("UNINSTALL_CONFIRM", {
        name: mod.manifest.name
    });

    const subtitle = getTranslation("BY", {
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
            const task = async () => {
                let enableDeps = false;
                const hasDisabledDeps = newVal
                    ? await commands.hasDisabledDeps({ uniqueName: mod.manifest.uniqueName })
                    : false;
                if (hasDisabledDeps) {
                    enableDeps =
                        autoEnableDeps ||
                        (await dialog.ask(getTranslation("ENABLE_DEPS_MESSAGE"), {
                            type: "info",
                            title: getTranslation("CONFIRM")
                        }));
                }
                const warnings = await commands.toggleMod({
                    uniqueName: mod.manifest.uniqueName,
                    enabled: newVal,
                    recursive: enableDeps
                });
                commands.refreshLocalDb();
                for (const modName of warnings) {
                    dialog.message(getTranslation("PREPATCHER_WARNING", { name: modName }), {
                        type: "warning",
                        title: getTranslation("PREPATCHER_WARNING_TITLE", {
                            name: modName
                        })
                    });
                }
            };
            task();
        },
        [mod.manifest.uniqueName, getTranslation, autoEnableDeps]
    );

    const onOpen = useCallback(() => {
        commands.openModFolder({ uniqueName: mod.manifest.uniqueName });
    }, [mod.manifest.uniqueName]);

    const onUninstall = useCallback(() => {
        confirm(uninstallConfirmText, getTranslation("CONFIRM")).then((answer) => {
            if (answer) {
                commands.uninstallMod({ uniqueName: mod.manifest.uniqueName }).then((warnings) => {
                    commands.refreshLocalDb();
                    for (const modName of warnings) {
                        dialog.message(getTranslation("PREPATCHER_WARNING", { name: modName }), {
                            type: "warning",
                            title: getTranslation("PREPATCHER_WARNING_TITLE", { name: modName })
                        });
                    }
                });
            }
        });
    }, [uninstallConfirmText, getTranslation, mod.manifest.uniqueName]);

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
