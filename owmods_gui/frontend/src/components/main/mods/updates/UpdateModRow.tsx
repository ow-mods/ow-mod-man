import { commands, hooks } from "@commands";
import { useGetTranslation, useUnifiedMod } from "@hooks";
import { memo, useCallback, useMemo } from "react";
import ModRow from "../ModRow";
import { UpdateRounded } from "@mui/icons-material";
import ModDownloadIcon from "../ModDownloadIcon";
import { RemoteMod } from "@types";

export interface UpdateModRowProps {
    uniqueName: string;
    hideThumbnail: boolean;
}

const UpdateModRow = memo(function UpdateModRow(props: UpdateModRowProps) {
    const getTranslation = useGetTranslation();

    // Fetch data
    const [status1, local] = hooks.getLocalMod("localRefresh", { ...props });
    const remoteOpt = hooks.getRemoteMod("remoteRefresh", { ...props })[1];

    const remote = (remoteOpt?.type === "err" ? null : remoteOpt?.data) as RemoteMod | null;

    // Transform data
    const { name, author, description, version, outdated, slug } = useUnifiedMod(local, remote);

    const onUpdate = useCallback(() => {
        commands.updateMod({ uniqueName: props.uniqueName }).then(() => commands.refreshLocalDb());
    }, [props.uniqueName]);

    const updateButton = useMemo(
        () => (
            <ModDownloadIcon
                uniqueName={remote?.uniqueName ?? ""}
                onClick={onUpdate}
                icon={<UpdateRounded />}
                tooltip={getTranslation("UPDATE")}
            />
        ),
        [getTranslation, onUpdate, remote?.uniqueName]
    );

    return (
        <ModRow
            uniqueName={props.uniqueName}
            name={name}
            slug={slug}
            requiresDlc={remote?.tags?.includes("requires-dlc") ?? false}
            author={author}
            version={version}
            hideThumbnail={props.hideThumbnail}
            isOutdated={outdated || props.uniqueName === "Alek.OWML"}
            isLoading={status1 === "Loading" && local === null}
            remoteIsLoading={(remoteOpt?.type ?? "loading") === "loading"}
            description={description}
            downloads={remote?.downloadCount ?? -1}
        >
            {updateButton}
        </ModRow>
    );
});

export default UpdateModRow;
