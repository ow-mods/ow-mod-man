import { commands, hooks } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import ModValidationModal, {
    ModValidationModalHandle,
    OpenModValidationModalPayload
} from "@components/modals/ModValidationModal";
import { useGetTranslation } from "@hooks";
import { memo, useCallback, useEffect, useRef, useState } from "react";
import UnsafeModRow from "./UnsafeModRow";

const LocalMods = memo(function LocalMods() {
    const validationModalRef = useRef<ModValidationModalHandle>();
    const [filter, setFilter] = useState("");
    const [tempFilter, setTempFilter] = useState("");
    const activeTimeout = useRef<number | undefined>(undefined);
    const [status, mods, err] = hooks.getLocalMods("LOCAL-REFRESH", { filter });
    const getTranslation = useGetTranslation();

    useEffect(() => {
        commands.refreshLocalDb();
    }, []);

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
        validationModalRef.current?.open(p);
    }, []);

    if (status === "Loading" && mods === null) {
        return <CenteredSpinner className="mod-list" />;
    } else if (status === "Error") {
        return <div className="center mod-list">{err!.toString()}</div>;
    } else {
        return (
            <>
                <ModValidationModal ref={validationModalRef} />
                {(filter.length >= 0 || mods!.length !== 0) && (
                    <div className="local-toolbar">
                        <input
                            className="search"
                            aria-label={getTranslation("SEARCH")}
                            placeholder={getTranslation("SEARCH")}
                            value={tempFilter}
                            onChange={(e) => onSearch(e.target.value)}
                        />
                        <div className="gap" />
                        <div className="local-mods-buttons">
                            <button onClick={() => onToggleAll(false)} className="secondary">
                                {getTranslation("DISABLE_ALL")}
                            </button>
                            <button onClick={() => onToggleAll(true)}>{getTranslation("ENABLE_ALL")}</button>
                        </div>
                    </div>
                )}
                <div className="mod-list">
                    {filter.length === 0 && mods!.length === 0 && (
                        <p className="center muted">{getTranslation("NO_MODS")}</p>
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
