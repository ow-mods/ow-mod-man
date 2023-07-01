import { commands, hooks } from "@commands";
import { memo, useCallback, useMemo, useState } from "react";
import ModsPage from "../ModsPage";
import UpdateModRow from "./UpdateModRow";
import { UpdateRounded } from "@mui/icons-material";
import { Button } from "@mui/material";
import { useGetTranslation } from "@hooks";

const UpdateModsPage = memo(function UpdateModsPage() {
    const getTranslation = useGetTranslation();

    const [filter, setFilter] = useState("");
    const [updatingAll, setUpdatingAll] = useState(false);

    const [status, updateMods] = hooks.getUpdatableMods(["LOCAL-REFRESH", "REMOTE-REFRESH"], {
        filter
    });

    const renderRow = useCallback((uniqueName: string) => {
        return <UpdateModRow uniqueName={uniqueName} />;
    }, []);

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
        <ModsPage
            isLoading={status === "Loading" && updateMods === null}
            actionsSize={130}
            noModsText={getTranslation("NO_UPDATES")}
            filter={filter}
            onFilterChange={setFilter}
            uniqueNames={updateMods ?? []}
            renderRow={renderRow}
        >
            {updateAllButton}
        </ModsPage>
    );
});

export default UpdateModsPage;
