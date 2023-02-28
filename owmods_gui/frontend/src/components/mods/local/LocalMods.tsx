import { commands, hooks } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import { useTranslations } from "@hooks";
import { memo, useCallback } from "react";
import LocalModRow from "./LocalModRow";

const LocalMods = memo(() => {
    const [status, mods, err] = hooks.getLocalMods("LOCAL-REFRESH");

    const [noMods, enableAll, disableAll] = useTranslations([
        "NO_MODS",
        "ENABLE_ALL",
        "DISABLE_ALL"
    ]);

    const onToggleAll = useCallback((enabled: boolean) => {
        commands
            .toggleAll({ enabled })
            .then(() => commands.refreshLocalDb())
            .catch(console.warn);
    }, []);

    if (status === "Loading" && mods === null) {
        return <CenteredSpinner className="mod-list" />;
    } else if (status === "Error") {
        return <div className="center mod-list">{err!.toString()}</div>;
    } else {
        return (
            <>
                {mods!.length !== 0 && (
                    <div className="local-mods-buttons">
                        <button onClick={() => onToggleAll(false)} className="secondary">
                            {disableAll}
                        </button>
                        <button onClick={() => onToggleAll(true)}>{enableAll}</button>
                    </div>
                )}
                <div className="mod-list">
                    {mods!.length === 0 && <p className="center muted">{noMods}</p>}
                    {mods!.map((m) => (
                        <LocalModRow key={m} uniqueName={m} />
                    ))}
                </div>
            </>
        );
    }
});

export default LocalMods;
