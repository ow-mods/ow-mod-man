import { commands, hooks } from "@commands";
import { useGetTranslation, useUnifiedMod } from "@hooks";
import { memo, useCallback, useMemo } from "react";
import ModRow from "../ModRow";
import { UpdateRounded } from "@mui/icons-material";
import ModDownloadIcon from "../ModDownloadIcon";

export interface UpdateModRowProps {
    uniqueName: string;
}

const UpdateModRow = memo(function UpdateModRow(props: UpdateModRowProps) {
    const getTranslation = useGetTranslation();

    // Fetch data
    const [status1, local] = hooks.getLocalMod("LOCAL-REFRESH", { ...props });
    const [status2, remote] = hooks.getRemoteMod("REMOTE-REFRESH", { ...props });

    // Transform data
    const { name, author, description, version, outdated } = useUnifiedMod(local, remote);

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
            author={author}
            version={version}
            isOutdated={outdated}
            isLoading={status1 === "Loading" && local === null}
            remoteIsLoading={status2 === "Loading" && remote === null}
            description={description}
            downloads={remote?.downloadCount ?? -1}
        >
            {updateButton}
        </ModRow>
    );
});

export default UpdateModRow;
