import { commands, hooks } from "@commands";
import { useGetTranslation, useUnifiedMod } from "@hooks";
import { memo, useCallback, useMemo } from "react";
import ModRow from "../ModRow";
import { UpdateRounded } from "@mui/icons-material";
import { Box, CircularProgress } from "@mui/material";
import ModActionIcon from "../ModActionIcon";

export interface UpdateModRowProps {
    uniqueName: string;
}

const UpdateModRow = memo(function UpdateModRow(props: UpdateModRowProps) {
    const getTranslation = useGetTranslation();

    // Fetch data
    const [status1, local] = hooks.getLocalMod("LOCAL-REFRESH", { ...props });
    const [status2, remote] = hooks.getRemoteMod("REMOTE-REFRESH", { ...props });
    const busy = hooks.getModBusy("MOD-BUSY", { uniqueName: props.uniqueName })[1];

    // Transform data
    const { name, author, description, version, outdated } = useUnifiedMod(local, remote);

    const onUpdate = useCallback(() => {
        commands.updateMod({ uniqueName: props.uniqueName }).then(() => commands.refreshLocalDb());
    }, [props.uniqueName]);

    const updateButton = useMemo(
        () =>
            busy ? (
                <Box display="flex" alignItems="center">
                    <CircularProgress color="inherit" size={22} />
                </Box>
            ) : (
                <ModActionIcon
                    onClick={onUpdate}
                    label={getTranslation("UPDATE")}
                    icon={<UpdateRounded />}
                />
            ),
        [busy, getTranslation, onUpdate]
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
