import { commands, hooks } from "@commands";
import { memo, useEffect, useState } from "react";
import ModsPage from "../ModsPage";
import LocalModRow from "./LocalModRow";
import LocalModsToggleButtons from "./LocalModsToggleButtons";

const LocalModsPage = memo(function LocalModsPage(props: { show: boolean }) {
    useEffect(() => {
        commands.refreshLocalDb();
    }, []);

    const [filter, setFilter] = useState("");

    const localMods = hooks.getLocalMods("LOCAL-REFRESH", { filter })[1];

    const onToggleAll = (newVal: boolean) => {
        commands.toggleAll({ enabled: newVal }).then(() => commands.refreshLocalDb());
    };

    return (
        <ModsPage
            show={props.show}
            filter={filter}
            onFilterChange={setFilter}
            uniqueNames={localMods ?? []}
            renderRow={(uniqueName) => <LocalModRow uniqueName={uniqueName} />}
            addToToolbar={<LocalModsToggleButtons onToggle={onToggleAll} />}
        />
    );
});

export default LocalModsPage;
