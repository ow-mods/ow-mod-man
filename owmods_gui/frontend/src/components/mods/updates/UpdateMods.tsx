import { commands, hooks } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import { useTauriCount, useGetTranslation } from "@hooks";
import { memo, useCallback, useState } from "react";
import UpdateModRow from "./UpdateModRow";

const UpdateMods = memo(function UpdateMods() {
    const [status, updates, err] = hooks.getUpdatableMods(["REMOTE-REFRESH", "LOCAL-REFRESH"]);
    const [updating, setUpdating] = useState(false);
    const modsUpdating = useTauriCount("INSTALL-START", "INSTALL-FINISH");
    const getTranslation = useGetTranslation();

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
            <>
                {updates!.length > 0 ? (
                    <>
                        <button
                            onClick={onUpdateAll}
                            className="update-all-button"
                            aria-busy={updating}
                            disabled={updating || modsUpdating > 0}
                        >
                            {updating ? getTranslation("UPDATING_ALL") : getTranslation("UPDATE_ALL")}
                        </button>
                        <div className="mod-list">
                            {updates!.map((m) => (
                                <UpdateModRow parentUpdating={updating} key={m} uniqueName={m} />
                            ))}
                        </div>
                    </>
                ) : (
                    <p className="center muted">{getTranslation("NO_UPDATES")}</p>
                )}
            </>
        );
    }
});

export default UpdateMods;
