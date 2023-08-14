import { Box } from "@mui/material";
import TopBar from "./top-bar/TopBar";
import { ReactNode, Suspense, lazy, memo, useCallback, useEffect, useState } from "react";
import { TabContext } from "@mui/lab";
import AppTabs, { ModsTab } from "./top-bar/AppTabs";
import LocalModsPage from "./mods/local/LocalModsPage";
import { TranslationKey, TranslationMap } from "@components/common/TranslationContext";
import { commands, hooks } from "@commands";
import { getCurrent } from "@tauri-apps/api/window";
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
                        <LocalModsPage
                            filter={filter}
                            onFilterChanged={setFilter}
                            tags={tags}
                            onTagsChanged={setTags}
                        />
                    </Pane>
                    <Pane
                        resetEvent="remoteRefresh"
                        errKey="NO_REMOTE_MODS"
                        show={selectedTab === "remote"}
                    >
                        <Suspense>
                            <RemoteModsPage
                                filter={filter}
                                onFilterChanged={setFilter}
                                tags={tags}
                                onTagsChanged={setTags}
                            />
                        </Suspense>
                    </Pane>
                    <Pane show={selectedTab === "updates"}>
                        <Suspense>
                            <UpdateModsPage filter={filter} onFilterChange={setFilter} />
                        </Suspense>
                    </Pane>
                </Box>
            </TabContext>
        </>
    );
});

const MainApp = () => {
    const [status, guiConfig] = hooks.getGuiConfig("guiConfigReload");

    const errorBound = useErrorBoundary();

    useEffect(() => {
        if (guiConfig?.language !== null) {
            getCurrent()
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
