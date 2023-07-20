import { commands, hooks } from "@commands";
import { memo, useEffect, useMemo } from "react";
import ModsPage from "../ModsPage";
import RemoteModRow from "./RemoteModRow";
import { Button } from "@mui/material";
import { useGetTranslation } from "@hooks";
import { PublicRounded } from "@mui/icons-material";
import { shell } from "@tauri-apps/api";
import { useErrorBoundary } from "react-error-boundary";

export interface RemoteModsPageProps {
    tags: string[];
    filter: string;
    onFilterChanged: (newVal: string) => void;
    onTagsChanged: (newVal: string[]) => void;
}

const RemoteModsPage = memo(function RemoteModsPage(props: RemoteModsPageProps) {
    const getTranslation = useGetTranslation();

    const errorBound = useErrorBoundary();

    useEffect(() => {
        commands.refreshRemoteDb({}, false).catch((e) => {
            errorBound.showBoundary(e?.toString() ?? getTranslation("UNKNOWN_ERROR"));
        });
    }, [errorBound, getTranslation]);

    const [status, remoteMods] = hooks.getRemoteMods("remoteRefresh", {
        filter: props.filter,
        tags: props.tags
    });

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
            actionsSize={100}
            noModsText={getTranslation("NO_REMOTE_MODS")}
            isLoading={status === "Loading" && remoteMods === null}
            filter={props.filter}
            onFilterChange={props.onFilterChanged}
            uniqueNames={remoteMods ?? []}
            renderRow={(uniqueName) => <RemoteModRow uniqueName={uniqueName} />}
            selectedTags={props.tags}
            onSelectedTagsChanged={props.onTagsChanged}
        >
            {modsWebsiteButton}
        </ModsPage>
    );
});

export default RemoteModsPage;
