import { memo, useCallback, useRef } from "react";
import ModRow from "../ModRow";
import { commands, hooks } from "@commands";
import { useGetTranslation } from "@hooks";
import { dialog } from "@tauri-apps/api";
import ModActionIcon from "../ModActionIcon";
import { DescriptionRounded, DownloadRounded, ScienceRounded } from "@mui/icons-material";
import { Box, CircularProgress } from "@mui/material";
import ModActionOverflow, { ModActionOverflowItem } from "../ModActionOverflow";

export interface RemoteModRowProps {
    uniqueName: string;
}

const RemoteModRow = memo(function RemoteModRow(props: RemoteModRowProps) {
    const getTranslation = useGetTranslation();

    const remote = hooks.getRemoteMod("REMOTE-REFRESH", { uniqueName: props.uniqueName })[1];
    const busy = hooks.getModBusy("MOD-BUSY", { uniqueName: props.uniqueName })[1];

    const usePrerelease = getTranslation("USE_PRERELEASE", {
        version: remote?.prerelease?.version ?? ""
    });

    const onInstall = useCallback(() => {
        commands
            .installMod({ uniqueName: props.uniqueName })
            .then(() => {
                commands.refreshLocalDb().catch(console.error);
            })
            .catch(console.error);
    }, [props.uniqueName]);

    const onPrerelease = useCallback(() => {
        const task = async () => {
            const result = await dialog.ask(getTranslation("PRERELEASE_WARNING"), {
                title: usePrerelease
            });
            if (result) {
                commands
                    .installMod({ uniqueName: props.uniqueName, prerelease: true })
                    .then(() => {
                        commands.refreshLocalDb().catch(console.error);
                    })
                    .catch(console.error);
            }
        };
        task();
    }, [getTranslation, usePrerelease, props.uniqueName]);

    const onReadme = useCallback(() => {
        commands.openModReadme({ uniqueName: props.uniqueName }).catch(console.warn);
    }, [props.uniqueName]);

    const overflowRef = useRef<{ onClose: () => void }>();

    return (
        <ModRow
            uniqueName={props.uniqueName}
            name={remote?.name ?? props.uniqueName}
            description={remote?.description}
            author={remote?.authorDisplay ?? remote?.author ?? ""}
            downloads={remote?.downloadCount ?? -1}
            version={remote?.version ?? "0.0.0"}
        >
            {busy ? (
                <Box display="flex" alignItems="center">
                    <CircularProgress color="inherit" size={22} />
                </Box>
            ) : (
                <ModActionIcon
                    onClick={onInstall}
                    label={getTranslation("INSTALL")}
                    icon={<DownloadRounded />}
                />
            )}
            <ModActionOverflow id={`remote-${props.uniqueName}`} ref={overflowRef}>
                <ModActionOverflowItem
                    label={getTranslation("OPEN_WEBSITE")}
                    icon={<DescriptionRounded />}
                    onClick={onReadme}
                    onClose={overflowRef.current?.onClose}
                />
                {remote?.prerelease && (
                    <ModActionOverflowItem
                        label={usePrerelease}
                        icon={<ScienceRounded />}
                        onClick={onPrerelease}
                        disabled={busy ?? false}
                        onClose={overflowRef.current?.onClose}
                    />
                )}
            </ModActionOverflow>
        </ModRow>
    );
});

export default RemoteModRow;
