import { Box } from "@mui/material";
import TopBar from "./top-bar/TopBar";
import { Suspense, lazy, useCallback, useEffect, useState } from "react";
import { TabContext } from "@mui/lab";
import AppTabs, { ModsTab } from "./top-bar/AppTabs";
import LocalModsPage from "./mods/local/LocalModsPage";
import { TranslationMap } from "@components/common/TranslationContext";
import { commands, hooks } from "@commands";
import { getCurrent } from "@tauri-apps/api/window";
import AppAlert from "./AppAlert";
import BaseApp from "@components/common/BaseApp";
import OwmlModal from "./OwmlModal";

const RemoteModsPage = lazy(() => import("./mods/remote/RemoteModsPage"));
const UpdateModsPage = lazy(() => import("./mods/updates/UpdateModsPage"));

const MainApp = () => {
    const [selectedTab, setSelectedTab] = useState<ModsTab>("local");
    const [status, guiConfig, err] = hooks.getGuiConfig("GUI_CONFIG_RELOAD");

    useEffect(() => {
        commands.initialSetup();
    }, []);

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
            fatalError={err?.toString()}
        >
            <OwmlModal />
            <TabContext value={selectedTab}>
                <TopBar />
                <AppAlert />
                <AppTabs onChange={onTabChange} />
                <Box display="flex" flexGrow={1} minHeight="0">
                    <LocalModsPage show={selectedTab === "local"} />
                    <Suspense>
                        <RemoteModsPage show={selectedTab === "remote"} />
                    </Suspense>
                    <Suspense>
                        <UpdateModsPage show={selectedTab === "updates"} />
                    </Suspense>
                </Box>
            </TabContext>
        </BaseApp>
    );
};

export default MainApp;
