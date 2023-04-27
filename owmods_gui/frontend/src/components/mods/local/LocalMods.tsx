import { commands, hooks } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import ModValidationModal, {
    OpenModValidationModalPayload
} from "@components/modals/ModValidationModal";
import { useTranslations } from "@hooks";
import { memo, useCallback, useEffect, useRef, useState } from "react";
import UnsafeModRow from "./UnsafeModRow";

const LocalMods = memo(function LocalMods() {
    const openValidationModal = useRef<(p: OpenModValidationModalPayload) => void>(() => null);
    const [filter, setFilter] = useState("");
    const [tempFilter, setTempFilter] = useState("");
    const activeTimeout = useRef<number | undefined>(undefined);
    const [status, mods, err] = hooks.getLocalMods("LOCAL-REFRESH", { filter });

    useEffect(() => {
        commands.refreshLocalDb();
    }, []);

    const [noMods, enableAll, disableAll, searchLabel] = useTranslations([
        "NO_MODS",
        "ENABLE_ALL",
        "DISABLE_ALL",
        "SEARCH"
    ]);

    const onToggleAll = useCallback((enabled: boolean) => {
        commands
            .toggleAll({ enabled })
            .then(() => commands.refreshLocalDb())
            .catch(console.warn);
    }, []);

    const onSearch = (newFilter: string) => {
        if (activeTimeout !== null) {
            clearTimeout(activeTimeout.current);
        }
        setTempFilter(newFilter);
        activeTimeout.current = setTimeout(() => {
            setFilter(newFilter);
        }, 450);
    };

    const onValidationIconClicked = useCallback((p: OpenModValidationModalPayload) => {
        openValidationModal.current(p);
    }, []);

    if (status === "Loading" && mods === null) {
        return <CenteredSpinner className="mod-list" />;
    } else if (status === "Error") {
        return <div className="center mod-list">{err!.toString()}</div>;
    } else {
        return (
            <>
                <ModValidationModal open={openValidationModal} />
                {(filter.length >= 0 || mods!.length !== 0) && (
                    <div className="local-toolbar">
                        <input
                            className="search"
                            aria-label={searchLabel}
                            placeholder={searchLabel}
                            value={tempFilter}
                            onChange={(e) => onSearch(e.target.value)}
                        />
                        <div className="gap" />
                        <div className="local-mods-buttons">
                            <button onClick={() => onToggleAll(false)} className="secondary">
                                {disableAll}
                            </button>
                            <button onClick={() => onToggleAll(true)}>{enableAll}</button>
                        </div>
                    </div>
                )}
                <div className="mod-list">
                    {filter.length === 0 && mods!.length === 0 && (
                        <p className="center muted">{noMods}</p>
                    )}
                    {filter !== tempFilter ? (
                        <CenteredSpinner />
                    ) : (
                        mods!.map((m) => (
                            <UnsafeModRow
                                key={m}
                                uniqueName={m}
                                onValidationClicked={onValidationIconClicked}
                            />
                        ))
                    )}
                </div>
            </>
        );
    }
});

export default LocalMods;
