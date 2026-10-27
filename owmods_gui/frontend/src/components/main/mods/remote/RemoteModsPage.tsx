import { commands, hooks } from "@commands";
import { memo, useCallback, useEffect, useMemo } from "react";
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
        const [recommendedStatus, recommendedMods] = hooks.getRecommendedMods([
            "remoteRefresh",
            "localRefresh"
        ]);

        const uniqueNames = useMemo(() => {
            const allMods = remoteMods ?? [];
            const recommendedNames = new Set((recommendedMods ?? []).map((mod) => mod.uniqueName));
            if (recommendedNames.size === 0) {
                return allMods;
            }
            const recommended: string[] = [];
            const others: string[] = [];
            for (const uniqueName of allMods) {
                (recommendedNames.has(uniqueName) ? recommended : others).push(uniqueName);
            }
            return [...recommended, ...others];
        }, [remoteMods, recommendedMods]);

        const recommendedByMap = useMemo(
            () =>
                new Map((recommendedMods ?? []).map((mod) => [mod.uniqueName, mod.recommendedBy])),
            [recommendedMods]
        );

        const renderRow = useCallback(
            (uniqueName: string, showThumbnail: boolean) => {
                const recommendedBy = recommendedByMap.get(uniqueName);
                return (
                    <RemoteModRow
                        showThumbnail={showThumbnail}
                        uniqueName={uniqueName}
                        recommendedBy={recommendedBy}
                    />
                );
            },
            [recommendedByMap]
        );

        const modsWebsiteButton = useMemo(
            () => (
                <Button
                    size="small"
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
                noModsText={getTranslation("NO_REMOTE_MODS")}
                isLoading={
                    (status === "Loading" && remoteMods === null) ||
                    remoteMods === undefined ||
                    (recommendedStatus === "Loading" && recommendedMods === null)
                }
                filter={props.filter}
                onFilterChange={props.onFilterChanged}
                uniqueNames={uniqueNames}
                renderRow={renderRow}
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
