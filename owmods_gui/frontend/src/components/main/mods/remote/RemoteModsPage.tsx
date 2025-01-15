import { commands, hooks } from "@commands";
import { memo, useEffect, useMemo } from "react";
import ModsPage from "../ModsPage";
import RemoteModRow from "./RemoteModRow";
import { Button } from "@mui/material";
import { useGetTranslation } from "@hooks";
import { PublicRounded } from "@mui/icons-material";
import * as shell from "@tauri-apps/plugin-shell";
import { useErrorBoundary } from "react-error-boundary";

export interface RemoteModsPageProps {
    tags: string[];
    filter: string;
    onFilterChanged: (newVal: string) => void;
    onTagsChanged: (newVal: string[]) => void;
}

const RemoteModsPage = memo(
    function RemoteModsPage(props: RemoteModsPageProps) {
        const getTranslation = useGetTranslation();
        const guiConfig = hooks.getGuiConfig("guiConfigReload")[1];

        const tags = useMemo(() => {
            return props.tags.filter((tag) => !guiConfig?.hideDlc || tag !== "requires-dlc");
        }, [props.tags, guiConfig?.hideDlc]);

        const errorBound = useErrorBoundary();

        useEffect(() => {
            commands.refreshRemoteDb({}, false).catch((e) => {
                errorBound.showBoundary(e?.toString() ?? getTranslation("UNKNOWN_ERROR"));
            });
        }, [errorBound, getTranslation]);

        const [status, remoteMods] = hooks.getRemoteMods(
            ["remoteRefresh", "localRefresh", "guiConfigReload"],
            {
                filter: props.filter,
                tags: tags
            }
        );

        const modsWebsiteButton = useMemo(
            () => (
                <Button
                    color="neutral"
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
                isLoading={
                    (status === "Loading" && remoteMods === null) || remoteMods === undefined
                }
                filter={props.filter}
                onFilterChange={props.onFilterChanged}
                uniqueNames={remoteMods ?? []}
                renderRow={(uniqueName, showThumbnail) => (
                    <RemoteModRow showThumbnail={showThumbnail} uniqueName={uniqueName} />
                )}
                selectedTags={tags}
                hideTags={guiConfig?.hideDlc ? ["requires-dlc"] : []}
                onSelectedTagsChanged={props.onTagsChanged}
            >
                {modsWebsiteButton}
            </ModsPage>
        );
    },
    (prev, next) => {
        return (
            prev.filter === next.filter &&
            prev.tags.length === next.tags.length &&
            prev.onFilterChanged === next.onFilterChanged &&
            prev.onTagsChanged === next.onTagsChanged
        );
    }
);

export default RemoteModsPage;
