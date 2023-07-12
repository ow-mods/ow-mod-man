import { memo, useCallback, useMemo } from "react";
import ModRow from "../ModRow";
import { commands, hooks } from "@commands";
import { useGetTranslation } from "@hooks";
import { dialog } from "@tauri-apps/api";
import RemoteModActions from "./RemoteModActions";

export interface RemoteModRowProps {
    uniqueName: string;
}

const RemoteModRow = memo(function RemoteModRow(props: RemoteModRowProps) {
    const getTranslation = useGetTranslation();

    const [status, remote] = hooks.getRemoteMod("remoteRefresh", { uniqueName: props.uniqueName });
    const busy = hooks.getModBusy("modBusy", { uniqueName: props.uniqueName })[1];

    const hasPrerelease = useMemo(() => remote?.prerelease !== undefined, [remote?.prerelease]);

    const prereleaseLabel = hasPrerelease
        ? getTranslation("USE_PRERELEASE", {
              version: remote?.prerelease?.version ?? ""
          })
        : "";

    const onInstall = useCallback(() => {
        commands
            .installMod({ uniqueName: props.uniqueName })
            .then(() => {
                commands.refreshLocalDb().catch(commands.logError);
            })
            .catch(commands.logError);
    }, [props.uniqueName]);

    const onPrerelease = useCallback(() => {
        const task = async () => {
            const result = await dialog.ask(getTranslation("PRERELEASE_WARNING"), {
                title: prereleaseLabel
            });
            if (result) {
                commands
                    .installMod({ uniqueName: props.uniqueName, prerelease: true })
                    .then(() => {
                        commands.refreshLocalDb().catch(commands.logError);
                    })
                    .catch(commands.logError);
            }
        };
        task();
    }, [getTranslation, prereleaseLabel, props.uniqueName]);

    const onReadme = useCallback(() => {
        commands.openModReadme({ uniqueName: props.uniqueName }).catch(commands.logError);
    }, [props.uniqueName]);

    const modActions = useMemo(
        () => (
            <RemoteModActions
                uniqueName={props.uniqueName}
                busy={busy ?? false}
                showPrerelease={hasPrerelease}
                prereleaseLabel={prereleaseLabel}
                onInstall={onInstall}
                onPrerelease={onPrerelease}
                onReadme={onReadme}
            />
        ),
        [busy, onInstall, onPrerelease, onReadme, prereleaseLabel, props.uniqueName, hasPrerelease]
    );

    return (
        <ModRow
            uniqueName={props.uniqueName}
            isLoading={status === "Loading" && remote === null}
            name={remote?.name ?? props.uniqueName}
            description={remote?.description}
            author={remote?.authorDisplay ?? remote?.author ?? ""}
            downloads={remote?.downloadCount ?? -1}
            version={remote?.version ?? "0.0.0"}
        >
            {modActions}
        </ModRow>
    );
});

export default RemoteModRow;
