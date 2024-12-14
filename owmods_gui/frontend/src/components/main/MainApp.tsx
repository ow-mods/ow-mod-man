import { lazy, memo, ReactNode, Suspense, useCallback, useEffect, useMemo, useState } from "react";
import { Box } from "@mui/material";
import TopBar from "./top-bar/TopBar";
import { TabContext } from "@mui/lab";
import AppTabs, { ModsTab } from "./top-bar/AppTabs";
import LocalModsPage from "./mods/local/LocalModsPage";
import { TranslationKey, TranslationMap } from "@components/common/TranslationContext";
import { commands, hooks } from "@commands";
import { getCurrentWindow } from "@tauri-apps/api/window";
import * as process from "@tauri-apps/plugin-process";
import AppAlert from "./AppAlert";
import BaseApp from "@components/common/BaseApp";
import OwmlModal from "./OwmlModal";
import StyledErrorBoundary from "@components/common/StyledErrorBoundary";
import { simpleOnError } from "../../errorHandling";
import { useErrorBoundary } from "react-error-boundary";
import FileDrop from "./FileDrop";
import { Event } from "@types";

const RemoteModsPage = lazy(() => import("./mods/remote/RemoteModsPage"));
const UpdateModsPage = lazy(() => import("./mods/updates/UpdateModsPage"));

const Pane = memo(function Pane(props: {
    show: boolean;
    errKey?: TranslationKey;
    resetEvent?: Event["name"];
    children: ReactNode;
}) {
    return (
        <Box width="100%" minHeight="0" display={props.show ? "flex" : "none"}>
            <StyledErrorBoundary
                resetEvent={props.resetEvent}
                errorKey={props.errKey ?? "PAGE_ERROR"}
                center
            >
                {props.children}
            </StyledErrorBoundary>
        </Box>
    );
});

const InnerMainApp = memo(function InnerMainApp() {
    const [selectedTab, setSelectedTab] = useState<ModsTab>("local");

    const [filter, setFilter] = useState("");
    const [tags, setTags] = useState<string[]>([]);

    const onTabChange = useCallback((newVal: string) => {
        setSelectedTab(newVal as ModsTab);
    }, []);

    const localModsPage = useMemo(
        () => (
            <LocalModsPage
                filter={filter}
                onFilterChanged={setFilter}
                tags={tags}
                onTagsChanged={setTags}
            />
        ),
        [filter, tags, setFilter, setTags]
    );

    const remoteModsPage = useMemo(
        () => (
            <Suspense>
                {" "}
                <RemoteModsPage
                    filter={filter}
                    onFilterChanged={setFilter}
                    tags={tags}
                    onTagsChanged={setTags}
                />{" "}
            </Suspense>
        ),
        [filter, tags, setFilter, setTags]
    );

    const updateModsPage = useMemo(
        () => (
            <Suspense>
                {" "}
                <UpdateModsPage filter={filter} onFilterChange={setFilter} />{" "}
            </Suspense>
        ),
        [filter, setFilter]
    );

    return (
        <>
            <OwmlModal />
            <FileDrop />
            <TabContext value={selectedTab}>
                <TopBar />
                <AppAlert />
                <AppTabs onChange={onTabChange} />
                <Box display="flex" flexGrow={1}>
                    <Pane resetEvent="localRefresh" show={selectedTab === "local"}>
                        {localModsPage}
                    </Pane>
                    <Pane
                        resetEvent="remoteRefresh"
                        errKey="NO_REMOTE_MODS"
                        show={selectedTab === "remote"}
                    >
                        {remoteModsPage}
                    </Pane>
                    <Pane show={selectedTab === "updates"}>{updateModsPage}</Pane>
                </Box>
            </TabContext>
        </>
    );
});

const thisWindow = getCurrentWindow();

thisWindow.onCloseRequested(() => {
    process.exit(0); // Exit the app on this window closing, so the log window doesn't stay open
});

const MainApp = () => {
    const [status, guiConfig] = hooks.getGuiConfig("guiConfigReload");

    const errorBound = useErrorBoundary();

    useEffect(() => {
        if (guiConfig?.language !== null) {
            thisWindow
                .setTitle(
                    TranslationMap[guiConfig?.language ?? "English"]["APP_TITLE"] ??
                        "Outer Wilds Mod Manager (*)"
                )
                .catch(simpleOnError);
        }
    }, [guiConfig?.language]);

    useEffect(() => {
        commands.initialSetup({}, false).catch((e) => errorBound.showBoundary(e));
    }, [errorBound]);

    return (
        <BaseApp
            language={guiConfig?.language}
            theme={guiConfig?.theme}
            usesRainbow={guiConfig?.rainbow}
            isLoading={status === "Loading" && guiConfig === null}
        >
            <InnerMainApp />
        </BaseApp>
    );
};

export default MainApp;
