import { commands, hooks } from "@commands";
import { memo, useCallback, useEffect, useMemo, useRef, useState } from "react";
import ModsPage from "../ModsPage";
import UpdateModRow from "./UpdateModRow";
import { UpdateRounded } from "@mui/icons-material";
import { Button } from "@mui/material";
import { useGetTranslation } from "@hooks";
import updateAudio from "@assets/update.ogg?url";

export interface UpdateModsPageProps {
    filter: string;
    onFilterChange: (newVal: string) => void;
}

const UpdateModsPage = memo(function UpdateModsPage(props: UpdateModsPageProps) {
    const getTranslation = useGetTranslation();

    const [updatingAll, setUpdatingAll] = useState(false);
    const updateAudioRef = useRef<HTMLAudioElement | null>(null);

    const guiConfig = hooks.getGuiConfig("guiConfigReload")[1];

    const [status, updateMods] = hooks.getUpdatableMods(["localRefresh", "remoteRefresh"], {
        filter: props.filter
    });

    const updatesAvailable = (updateMods?.length ?? 1) !== 0;

    useEffect(() => {
        if (updatesAvailable && updateAudioRef.current !== null) {
            updateAudioRef.current.currentTime = 0;
            updateAudioRef.current.play();
        } else {
            updateAudioRef.current?.pause?.();
        }
    }, [updatesAvailable]);

    const renderRow = useCallback(
        (uniqueName: string) => {
            return (
                <UpdateModRow
                    hideThumbnail={guiConfig?.hideModThumbnails ?? false}
                    uniqueName={uniqueName}
                />
            );
        },
        [guiConfig?.hideModThumbnails]
    );

    const onUpdateAll = useCallback(() => {
        setUpdatingAll(true);
        commands
            .updateAll({ uniqueNames: updateMods ?? [] })
            .then(() => commands.refreshLocalDb())
            .finally(() => setUpdatingAll(false));
    }, [updateMods]);

    const updateAllButton = useMemo(
        () => (
            <Button
                color="neutral"
                disabled={updateMods?.length === 0 || updatingAll}
                onClick={onUpdateAll}
                startIcon={<UpdateRounded />}
            >
                {updatingAll ? getTranslation("UPDATING_ALL") : getTranslation("UPDATE_ALL")}
            </Button>
        ),
        [updateMods?.length, updatingAll, onUpdateAll, getTranslation]
    );

    return (
        <>
            <audio style={{ display: "none" }} ref={updateAudioRef} src={updateAudio} />
            <ModsPage
                isLoading={status === "Loading" && updateMods === null}
                actionsSize={130}
                noModsText={getTranslation("NO_UPDATES")}
                filter={props.filter}
                onFilterChange={props.onFilterChange}
                uniqueNames={updateMods ?? []}
                renderRow={renderRow}
            >
                {updateAllButton}
            </ModsPage>
        </>
    );
});

export default UpdateModsPage;
