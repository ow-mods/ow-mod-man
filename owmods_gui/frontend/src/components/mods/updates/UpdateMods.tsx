import { commands, hooks } from "@commands";
import { useTranslations } from "@hooks";
import { memo, useCallback, useState } from "react";
import UpdateModRow from "./UpdateModRow";

const UpdateMods = memo(() => {
    const [status, updates, err] = hooks.get_updatable_mods(["REMOTE-REFRESH", "LOCAL-REFRESH"]);
    const [updating, setUpdating] = useState(false);

    const [updateAll, noUpdates] = useTranslations(["UPDATE_ALL", "NO_UPDATES"]);

    const onUpdateAll = useCallback(() => {
        setUpdating(true);
        commands
            .update_all_mods({ uniqueNames: updates ?? [] })
            .then(() => {
                commands.refresh_local_db().catch(console.warn);
            })
            .catch(console.warn)
            .finally(() => setUpdating(false));
    }, [updates]);

    if (status === "Loading" && updates === null) {
        return <div aria-busy className="mod-list center-loading"></div>;
    } else if (status === "Error") {
        return <div className="center-loading mod-list">{err!.toString()}</div>;
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
                    <p className="center-loading muted">{noUpdates}</p>
                )}
                {updates!.map((m) => (
                    <UpdateModRow key={m} uniqueName={m} />
                ))}
            </div>
        );
    }
});

export default UpdateMods;
