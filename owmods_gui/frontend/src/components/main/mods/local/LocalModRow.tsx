import { commands, hooks } from "@commands";
import ModRow from "../ModRow";
import { memo, useCallback, useMemo } from "react";
import { useGetTranslation, useUnifiedMod } from "@hooks";
import * as dialog from "@tauri-apps/plugin-dialog";
import * as shell from "@tauri-apps/plugin-shell";
import LocalModActions from "./LocalModActions";
import { LocalMod, RemoteMod, UnsafeLocalMod } from "@types";

const getErrorLevel = (mod?: UnsafeLocalMod): "err" | "warn" | undefined => {
    if (mod?.loadState === "invalid") {
        return "err";
    } else if (mod?.mod.errors.length !== 0) {
        if (
            mod?.mod.errors.find(
                (e) => e.errorType === "InvalidManifest" || e.errorType === "DuplicateMod"
            )
        ) {
            return "err";
        } else if (mod?.mod.enabled) {
            return "warn";
        }
    }
};

const getDisplayErrors = (
    getTranslation: ReturnType<typeof useGetTranslation>,
    mod?: UnsafeLocalMod
): string[] => {
    let errors: string[] = [];

    if (mod) {
        if (mod.loadState === "invalid") {
            errors.push(
                getTranslation(mod.mod.error.errorType, { payload: mod.mod.error.payload ?? "" })
            );
        } else {
            errors = mod.mod.errors.map((e) =>
                getTranslation(e.errorType, { payload: e.payload ?? "" })
            );
        }
    }

    return errors;
};

const canFix = (mod?: UnsafeLocalMod): boolean => {
    if (
        mod === undefined ||
        mod.loadState === "invalid" ||
        mod.mod.errors.length === 0 ||
        !mod.mod.enabled
    ) {
        return false;
    }

    return (mod.mod as LocalMod).errors.every(
        (e) =>
            e.errorType === "MissingDep" ||
            e.errorType === "DisabledDep" ||
            e.errorType === "Outdated"
    );
};

export interface LocalModRowProps {
    uniqueName: string;
    hideThumbnail: boolean;
}

const LocalModRow = memo(function LocalModRow(props: LocalModRowProps) {
    // Simple hooks
    const getTranslation = useGetTranslation();

    // Fetch data
    const [status1, local] = hooks.getLocalMod("localRefresh", { ...props });
    const remoteOpt = hooks.getRemoteMod("remoteRefresh", { ...props })[1];
    const autoEnableDeps = hooks.getGuiConfig("guiConfigReload")[1]?.autoEnableDeps ?? false;

    const remote = (remoteOpt?.type === "err" ? null : remoteOpt?.data) as RemoteMod | null;

    // Transform data
    const { name, slug, author, description, version, outdated, enabled } = useUnifiedMod(
        local,
        remote
    );
    const errorLevel = useMemo(() => getErrorLevel(local ?? undefined), [local]);
    const isErr = errorLevel === "err";
    const canFixWarn = useMemo(() => canFix(local ?? undefined), [local]);
    const displayErrors = useMemo(
        () => getDisplayErrors(getTranslation, local ?? undefined),
        [local, getTranslation]
    );
    const hasRemote = remote !== null;

    // Event Handlers
    const onReadme = useCallback(
        () => commands.openModReadme({ uniqueName: props.uniqueName }),
        [props.uniqueName]
    );
    const onFolder = useCallback(
        () => commands.openModFolder({ uniqueName: props.uniqueName }),
        [props.uniqueName]
    );
    const onGithub = useCallback(
        () => commands.openModGithub({ uniqueName: props.uniqueName }),
        [props.uniqueName]
    );
    const onToggle = useCallback(
        (newVal: boolean) => {
            const task = async () => {
                let enableDeps = false;
                const hasDisabledDeps = newVal
                    ? await commands.hasDisabledDeps({ uniqueName: props.uniqueName })
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
                    uniqueName: props.uniqueName,
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
        [autoEnableDeps, getTranslation, props.uniqueName]
    );
    const onUninstall = useCallback(() => {
        commands.uninstallMod({ uniqueName: props.uniqueName }).then((warnings) => {
            commands.refreshLocalDb();
            for (const modName of warnings) {
                dialog.message(getTranslation("PREPATCHER_WARNING", { name: modName }), {
                    type: "warning",
                    title: getTranslation("PREPATCHER_WARNING_TITLE", { name: modName })
                });
            }
        });
    }, [getTranslation, props.uniqueName]);
    const onFix = useCallback(() => {
        const task = async () => {
            if (outdated) {
                await commands.updateMod({ uniqueName: props.uniqueName });
            }
            await commands.fixDeps({ uniqueName: props.uniqueName });
            await commands.refreshLocalDb();
        };
        task();
    }, [outdated, props.uniqueName]);

    const donateLinks = (local?.mod as LocalMod)?.manifest?.donateLinks;

    const modsToolbar = useMemo(
        () => (
            <LocalModActions
                uniqueName={props.uniqueName}
                enabled={enabled}
                isErr={isErr}
                hasRemote={hasRemote}
                donateLinks={donateLinks}
                canFix={canFixWarn}
                onToggle={onToggle}
                onReadme={onReadme}
                onFix={onFix}
                onFolder={onFolder}
                onUninstall={onUninstall}
                onGithub={onGithub}
            />
        ),
        [
            props.uniqueName,
            enabled,
            isErr,
            hasRemote,
            canFixWarn,
            donateLinks,
            onToggle,
            onReadme,
            onFix,
            onFolder,
            onUninstall,
            onGithub
        ]
    );

    return (
        <ModRow
            uniqueName={props.uniqueName}
            name={name}
            slug={slug}
            alignActions="end"
            thumbnailUrl={remote?.thumbnail?.openGraph ?? remote?.thumbnail.main}
            thumbnailClasses={enabled ? "" : "disabled"}
            author={author}
            version={version}
            hideThumbnail={props.hideThumbnail}
            requiresDlc={remote?.tags?.includes("requires-dlc") ?? false}
            isOutdated={outdated}
            isLoading={status1 === "Loading" && local === null}
            remoteIsLoading={(remoteOpt?.type ?? "loading") === "loading"}
            description={
                (local?.loadState === "valid" && !enabled) || displayErrors.length === 0
                    ? description
                    : displayErrors.join("\n") +
                      (canFixWarn ? `\n${getTranslation("VALIDATION_FIX_MESSAGE")}` : "")
            }
            downloads={remote?.downloadCount ?? -1}
            errorLevel={errorLevel}
        >
            {modsToolbar}
        </ModRow>
    );
});

export default LocalModRow;
