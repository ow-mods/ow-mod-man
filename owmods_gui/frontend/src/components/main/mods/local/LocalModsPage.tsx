import { commands, hooks } from "@commands";
import { memo, useCallback, useEffect, useMemo, useState } from "react";
import ModsPage from "../ModsPage";
import LocalModRow from "./LocalModRow";
import LocalModsToggleButtons from "./LocalModsToggleButtons";
import { useGetTranslation } from "@hooks";

const LocalModsPage = memo(function LocalModsPage(props: { show: boolean }) {
    useEffect(() => {
        commands.refreshLocalDb();
    }, []);

    const getTranslation = useGetTranslation();

    const [filter, setFilter] = useState("");

    const [status, localMods] = hooks.getLocalMods("LOCAL-REFRESH", { filter });

    const onToggleAll = useCallback((newVal: boolean) => {
        commands.toggleAll({ enabled: newVal }).then(() => commands.refreshLocalDb());
    }, []);

    const renderRow = useCallback((uniqueName: string) => {
        return <LocalModRow uniqueName={uniqueName} />;
    }, []);

    const toggleButtons = useMemo(
        () => <LocalModsToggleButtons onToggle={onToggleAll} />,
        [onToggleAll]
    );

    return (
        <ModsPage
            isLoading={status === "Loading" && localMods === null}
            actionsSize={130}
            noModsText={getTranslation("NO_MODS")}
            show={props.show}
            filter={filter}
            onFilterChange={setFilter}
            uniqueNames={localMods ?? []}
            renderRow={renderRow}
        >
            {toggleButtons}
        </ModsPage>
    );
});

export default LocalModsPage;
