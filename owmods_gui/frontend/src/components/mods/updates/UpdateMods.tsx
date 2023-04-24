import { commands, hooks } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import { useTauriCount, useTranslations } from "@hooks";
import { memo, useCallback, useState } from "react";
import UpdateModRow from "./UpdateModRow";

const UpdateMods = memo(() => {
    const [status, updates, err] = hooks.getUpdatableMods(["REMOTE-REFRESH", "LOCAL-REFRESH"]);
    const [updating, setUpdating] = useState(false);
    const modsUpdating = useTauriCount("INSTALL-START", "INSTALL-FINISH");

    const [updateAll, noUpdates, updatingAll] = useTranslations([
        "UPDATE_ALL",
        "NO_UPDATES",
        "UPDATING_ALL"
    ]);

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
        return <CenteredSpinner className="center mod-list" />;
    } else if (status === "Error") {
        return <div className="center">{err!.toString()}</div>;
    } else {
        return (
            <div className="mod-list">
                {updates!.length > 0 ? (
                    <button
                        onClick={onUpdateAll}
                        className="update-all-button"
                        aria-busy={updating}
                        disabled={updating || modsUpdating > 0}
                    >
                        {updating ? updatingAll : updateAll}
                    </button>
                ) : (
                    <p className="center muted">{noUpdates}</p>
                )}
                {updates!.map((m) => (
                    <UpdateModRow parentUpdating={updating} key={m} uniqueName={m} />
                ))}
            </div>
        );
    }
});

export default UpdateMods;
