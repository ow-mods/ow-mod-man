import { commands, hooks } from "@commands";
import { memo, useCallback, useEffect, useMemo } from "react";
import ModsPage from "../ModsPage";
import LocalModRow from "./LocalModRow";
import LocalModsToggleButtons from "./LocalModsToggleButtons";
import { useGetTranslation } from "@hooks";
import { TableCell, Typography, useTheme } from "@mui/material";

export interface LocalModsPageProps {
    tags: string[];
    filter: string;
    onFilterChanged: (newVal: string) => void;
    onTagsChanged: (newVal: string[]) => void;
}

const LocalModsPage = memo(function LocalModsPage(props: LocalModsPageProps) {
    useEffect(() => {
        commands.refreshLocalDb();
    }, []);

    const theme = useTheme();
    const getTranslation = useGetTranslation();

    const [status, localMods] = hooks.getLocalMods("localRefresh", {
        filter: props.filter,
        tags: props.tags
    });

    const onToggleAll = useCallback((newVal: boolean) => {
        commands.toggleAll({ enabled: newVal }).then(() => commands.refreshLocalDb());
    }, []);

    const renderRow = useCallback(
        (uniqueName: string) => {
            return uniqueName === "~~SEPARATOR~~" ? (
                <TableCell sx={{ bgcolor: theme.palette.background.default }} colSpan={4}>
                    <Typography>{getTranslation("DISABLED_MODS")}</Typography>
                </TableCell>
            ) : (
                <LocalModRow uniqueName={uniqueName} />
            );
        },
        [getTranslation, theme.palette.background.default]
    );

    const toggleButtons = useMemo(
        () => <LocalModsToggleButtons onToggle={onToggleAll} />,
        [onToggleAll]
    );

    return (
        <ModsPage
            isLoading={status === "Loading" && localMods === null}
            actionsSize={130}
            noModsText={getTranslation("NO_MODS")}
            filter={props.filter}
            onFilterChange={props.onFilterChanged}
            uniqueNames={localMods ?? []}
            renderRow={renderRow}
            selectedTags={props.tags}
            onSelectedTagsChanged={props.onTagsChanged}
        >
            {toggleButtons}
        </ModsPage>
    );
});

export default LocalModsPage;
