import { memo, useCallback, useMemo } from "react";
import ModRow from "../ModRow";
import { commands, hooks } from "@commands";
import { useGetTranslation } from "@hooks";
import { dialog } from "@tauri-apps/api";
import RemoteModActions from "./RemoteModActions";
import { simpleOnError } from "../../../../errorHandling";
import { RemoteMod } from "@types";

export interface RemoteModRowProps {
    uniqueName: string;
}

const RemoteModRow = memo(function RemoteModRow(props: RemoteModRowProps) {
    const getTranslation = useGetTranslation();

    const [status, remoteOpt] = hooks.getRemoteMod("remoteRefresh", {
        uniqueName: props.uniqueName
    });
    const busy = hooks.getModBusy("modBusy", { uniqueName: props.uniqueName })[1];

    const remote = (remoteOpt?.type === "err" ? null : remoteOpt?.data) as RemoteMod | null;

    const hasPrerelease = remote?.prerelease !== undefined && remote?.prerelease !== null;

    const prereleaseLabel = hasPrerelease
        ? getTranslation("USE_PRERELEASE", {
              version: remote?.prerelease?.version ?? ""
          })
        : "";

    const onInstall = useCallback(() => {
        commands
            .installMod({ uniqueName: props.uniqueName })
            .then(() => {
                commands.refreshLocalDb().catch(simpleOnError);
            })
            .catch(simpleOnError);
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
                        commands.refreshLocalDb().catch(simpleOnError);
                    })
                    .catch(simpleOnError);
            }
        };
        task();
    }, [getTranslation, prereleaseLabel, props.uniqueName]);

    const onReadme = useCallback(() => {
        commands.openModReadme({ uniqueName: props.uniqueName }).catch(simpleOnError);
    }, [props.uniqueName]);

    const onGithub = useCallback(() => {
        commands.openModGithub({ uniqueName: props.uniqueName }).catch(simpleOnError);
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
                onGithub={onGithub}
            />
        ),
        [
            busy,
            onInstall,
            onPrerelease,
            onReadme,
            onGithub,
            prereleaseLabel,
            props.uniqueName,
            hasPrerelease
        ]
    );

    const isLoading =
        remoteOpt?.type === "loading" || (status === "Loading" && (remote ?? null) === null);

    return (
        <ModRow
            uniqueName={props.uniqueName}
            isLoading={isLoading}
            name={remote?.name ?? props.uniqueName}
            slug={remote?.slug}
            description={remote?.description}
            requiresDlc={remote?.tags?.includes("requires-dlc") ?? false}
            author={remote?.authorDisplay ?? remote?.author ?? ""}
            downloads={remote?.downloadCount ?? -1}
            version={remote?.version ?? "0.0.0"}
        >
            {modActions}
        </ModRow>
    );
});

export default RemoteModRow;
