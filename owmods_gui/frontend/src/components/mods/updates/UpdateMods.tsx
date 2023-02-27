import { commands, hooks } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import { useTranslations } from "@hooks";
import { memo, useCallback, useState } from "react";
import UpdateModRow from "./UpdateModRow";

const UpdateMods = memo(() => {
    const [status, updates, err] = hooks.getUpdatableMods(["REMOTE-REFRESH", "LOCAL-REFRESH"]);
    const [updating, setUpdating] = useState(false);

    const [updateAll, noUpdates] = useTranslations(["UPDATE_ALL", "NO_UPDATES"]);

    const onUpdateAll = useCallback(() => {
        setUpdating(true);
        commands
            .updateAll({ uniqueNames: updates ?? [] })
            .then(() => {
                commands.refreshLocalDb().catch(console.warn);
            })
            .catch(console.warn)
            .finally(() => setUpdating(false));
    }, [updates]);

    if (status === "Loading" && updates === null) {
        return <CenteredSpinner className="mod-list" />;
    } else if (status === "Error") {
        return <div className="center mod-list">{err!.toString()}</div>;
    } else {
        return (
            <div className="mod-list">
                {updates!.length > 0 ? (
                    <button
                        onClick={onUpdateAll}
                        className="update-all-button"
                        aria-busy={updating}
                    >
                        {updateAll}
                    </button>
                ) : (
                    <p className="center muted">{noUpdates}</p>
                )}
                {updating ? (
                    <CenteredSpinner />
                ) : (
                    updates!.map((m) => <UpdateModRow key={m} uniqueName={m} />)
                )}
            </div>
        );
    }
});

export default UpdateMods;
