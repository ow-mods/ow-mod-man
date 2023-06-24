import { commands, hooks } from "@commands";
import { memo, useEffect, useMemo, useState } from "react";
import ModsPage from "../ModsPage";
import RemoteModRow from "./RemoteModRow";
import { Button } from "@mui/material";
import { useGetTranslation } from "@hooks";
import { PublicRounded } from "@mui/icons-material";
import { shell } from "@tauri-apps/api";

const RemoteModsPage = memo(function RemoteModsPage(props: { show: boolean }) {
    const getTranslation = useGetTranslation();

    useEffect(() => {
        commands.refreshRemoteDb({}, false).catch(console.warn);
    }, []);

    const [filter, setFilter] = useState("");

    const [status, remoteMods] = hooks.getRemoteMods("REMOTE-REFRESH", { filter });

    const modsWebsiteButton = useMemo(
        () => (
            <Button
                onClick={() => shell.open("https://outerwildsmods.com/mods")}
                startIcon={<PublicRounded />}
            >
                {getTranslation("OPEN_WEBSITE")}
            </Button>
        ),
        [getTranslation]
    );

    return (
        <ModsPage
            show={props.show}
            actionsSize={100}
            noModsText={getTranslation("NO_REMOTE_MODS")}
            isLoading={status === "Loading" && remoteMods === null}
            filter={filter}
            onFilterChange={setFilter}
            uniqueNames={remoteMods ?? []}
            renderRow={(uniqueName) => <RemoteModRow uniqueName={uniqueName} />}
        >
            {modsWebsiteButton}
        </ModsPage>
    );
});

export default RemoteModsPage;
