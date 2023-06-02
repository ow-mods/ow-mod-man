import { commands, hooks } from "@commands";
import ModRow from "../ModRow";
import { LocalMod, UnsafeLocalMod } from "@types";
import { memo, useMemo, useRef } from "react";
import { Checkbox } from "@mui/material";
import ModActionIcon from "../ModActionIcon";
import { useGetTranslation } from "@hooks";
import {
    DeleteRounded,
    DescriptionRounded,
    FolderRounded
    // UpdateRounded
} from "@mui/icons-material";
import { dialog } from "@tauri-apps/api";
import ModActionOverflow, { ModActionOverflowItem } from "../ModActionOverflow";

export interface LocalModRowProps {
    uniqueName: string;
}

const safeOrNull = (mod: UnsafeLocalMod | null) => {
    if (mod === null) return null;
    if (mod.loadState === "invalid") {
        return null;
    } else {
        return mod.mod as LocalMod;
    }
};

const LocalModRow = memo(function LocalModRow(props: LocalModRowProps) {
    // Simple hooks
    const getTranslation = useGetTranslation();

    // Fetch data
    const [status1, local] = hooks.getLocalMod("LOCAL-REFRESH", { ...props });
    const [status2, remote] = hooks.getRemoteMod("REMOTE-REFRESH", { ...props });
    const autoEnableDeps = hooks.getGuiConfig("GUI_CONFIG_RELOAD")[1]?.autoEnableDeps ?? false;

    // Transform data
    const name = useMemo(
        () => remote?.name ?? safeOrNull(local)?.manifest.name ?? "",
        [local, remote]
    );
    const author = useMemo(
        () => remote?.authorDisplay ?? remote?.author ?? safeOrNull(local)?.manifest.author ?? "",
        [local, remote]
    );

    const description = remote?.description;

    const version = useMemo(() => safeOrNull(local)?.manifest.version ?? "--", [local]);

    const enabled = safeOrNull(local)?.enabled ?? false;

    const outdated = useMemo(
        () =>
            safeOrNull(local)?.errors.filter((e) => e.errorType === "Outdated").length !== 0 ??
            false,
        [local]
    );

    // Event Handlers
    const onReadme = () => commands.openModReadme({ uniqueName: props.uniqueName });
    const onFolder = () => commands.openModFolder({ uniqueName: props.uniqueName });
    const onToggle = (newVal: boolean) => {
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
    };
    const onUninstall = () => {
        const uninstallConfirmText = getTranslation("UNINSTALL_CONFIRM", {
            name: safeOrNull(local)?.manifest.name ?? "Unknown"
        });
        dialog.confirm(uninstallConfirmText, getTranslation("CONFIRM")).then((answer) => {
            if (answer) {
                commands.uninstallMod({ uniqueName: props.uniqueName }).then((warnings) => {
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
    };
    // const onUpdate = () => {
    //     commands
    //         .updateMod({ uniqueName: props.uniqueName })
    //         .then(() => {
    //             commands.refreshLocalDb().catch(console.warn);
    //         })
    //         .catch(console.error);
    // };

    const overflowRef = useRef<{ onClose: () => void }>();

    return (
        <ModRow
            uniqueName={props.uniqueName}
            name={name}
            author={author}
            version={version}
            isOutdated={outdated}
            isLoading={status1 === "Loading" && local === null}
            remoteIsLoading={status2 === "Loading" && remote === null}
            description={description}
            downloads={remote?.downloadCount ?? -1}
        >
            <Checkbox
                onChange={(e) => onToggle(e.target.checked)}
                color="default"
                checked={enabled}
            />
            {/* {outdated && (
                <ModActionIcon
                    icon={<UpdateRounded color="secondary" />}
                    label={getTranslation("UPDATE")}
                    onClick={onUpdate}
                />
            )} */}
            <ModActionIcon
                onClick={onReadme}
                label={getTranslation("OPEN_WEBSITE")}
                icon={<DescriptionRounded />}
            />
            <ModActionOverflow id={`local-${props.uniqueName}`} ref={overflowRef}>
                <ModActionOverflowItem
                    label={getTranslation("SHOW_FOLDER")}
                    icon={<FolderRounded />}
                    onClick={onFolder}
                    onClose={overflowRef.current?.onClose}
                />
                <ModActionOverflowItem
                    label={getTranslation("UNINSTALL")}
                    icon={<DeleteRounded />}
                    onClick={onUninstall}
                    onClose={overflowRef.current?.onClose}
                />
            </ModActionOverflow>
        </ModRow>
    );
});

export default LocalModRow;
