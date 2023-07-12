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
import { useErrorBoundary } from "react-error-boundary";
import FileDrop from "./FileDrop";

const RemoteModsPage = lazy(() => import("./mods/remote/RemoteModsPage"));
const UpdateModsPage = lazy(() => import("./mods/updates/UpdateModsPage"));

const Pane = memo(function Pane(props: {
    show: boolean;
    errKey?: TranslationKey;
    resetEvent?: string;
    children: ReactNode;
}) {
    return (
        <Box width="100%" display={props.show ? undefined : "none"}>
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

const MainApp = () => {
    const [selectedTab, setSelectedTab] = useState<ModsTab>("local");
    const [status, guiConfig] = hooks.getGuiConfig("GUI_CONFIG_RELOAD");

    const errorBound = useErrorBoundary();

    useEffect(() => {
        commands.initialSetup({}, false).catch((e) => errorBound.showBoundary(e));
    }, [errorBound]);

    useEffect(() => {
        if (guiConfig?.language !== null) {
            getCurrent()
                .setTitle(
                    TranslationMap[guiConfig?.language ?? "English"]["APP_TITLE"] ??
                        "Outer Wilds Mod Manager (*)"
                )
                .catch(console.warn);
        }
    }, [guiConfig?.language]);

    const onTabChange = useCallback((newVal: string) => {
        setSelectedTab(newVal as ModsTab);
    }, []);

    return (
        <BaseApp
            language={guiConfig?.language}
            isLoading={status === "Loading" && guiConfig === null}
        >
            <OwmlModal />
            <FileDrop />
            <TabContext value={selectedTab}>
                <TopBar />
                <AppAlert />
                <AppTabs onChange={onTabChange} />
                <Box display="flex" flexGrow={1} minHeight="0">
                    <Pane resetEvent="LOCAL-REFRESH" show={selectedTab === "local"}>
                        <LocalModsPage />
                    </Pane>
                    <Pane
                        resetEvent="REMOTE-REFRESH"
                        errKey="NO_REMOTE_MODS"
                        show={selectedTab === "remote"}
                    >
                        <Suspense>
                            <RemoteModsPage />
                        </Suspense>
                    </Pane>
                    <Pane show={selectedTab === "updates"}>
                        <Suspense>
                            <UpdateModsPage />
                        </Suspense>
                    </Pane>
                </Box>
            </TabContext>
        </BaseApp>
    );
};

export default MainApp;
