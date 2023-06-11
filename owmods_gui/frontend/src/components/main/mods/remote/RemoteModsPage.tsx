import { commands, hooks } from "@commands";
import { memo, useEffect, useState } from "react";
import ModsPage from "../ModsPage";
import RemoteModRow from "./RemoteModRow";

const RemoteModsPage = memo(function RemoteModsPage(props: { show: boolean }) {
    useEffect(() => {
        commands.refreshRemoteDb();
    }, []);

    const [filter, setFilter] = useState("");

    const [status, remoteMods] = hooks.getRemoteMods("REMOTE-REFRESH", { filter });

    return (
        <ModsPage
            show={props.show}
            actionsSize={100}
            isLoading={status === "Loading" && remoteMods === null}
            filter={filter}
            onFilterChange={setFilter}
            uniqueNames={remoteMods ?? []}
            renderRow={(uniqueName) => <RemoteModRow uniqueName={uniqueName} />}
        />
    );
});

export default RemoteModsPage;
