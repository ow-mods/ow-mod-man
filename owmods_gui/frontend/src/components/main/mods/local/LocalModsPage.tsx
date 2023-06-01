import { hooks } from "@commands";
import { useState } from "react";
import ModsPage from "../ModsPage";
import LocalModRow from "./LocalModRow";

const LocalModsPage = (props: { show: boolean }) => {
    const [filter, setFilter] = useState("");

    const localMods = hooks.getLocalMods("LOCAL_REFRESH", { filter })[1];

    return (
        <ModsPage
            show={props.show}
            filter={filter}
            onFilterChange={setFilter}
            uniqueNames={localMods ?? []}
            renderRow={(uniqueName) => <LocalModRow key={uniqueName} uniqueName={uniqueName} />}
        />
    );
};

export default LocalModsPage;
