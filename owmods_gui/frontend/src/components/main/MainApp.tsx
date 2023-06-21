import { Box, Paper } from "@mui/material";
import TopBar from "./top-bar/TopBar";
import { useEffect, useState } from "react";
import { TabContext } from "@mui/lab";
import AppTabs from "./top-bar/AppTabs";
import LocalModsPage from "./mods/local/LocalModsPage";
import RemoteModsPage from "./mods/remote/RemoteModsPage";
import UpdateModsPage from "./mods/updates/UpdateModsPage";
import { TranslationMap } from "@components/common/TranslationContext";
import { commands, hooks } from "@commands";
import { getCurrent } from "@tauri-apps/api/window";
import AppAlert from "./AppAlert";
import BaseApp from "@components/common/BaseApp";
import OwmlModal from "./OwmlModal";

const MainApp = () => {
    const [selectedTab, setSelectedTab] = useState("1");
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

    return (
        <BaseApp
            language={guiConfig?.language}
            isLoading={status === "Loading"}
            fatalError={err?.toString()}
        >
            <OwmlModal />
            <TabContext value={selectedTab}>
                <TopBar />
                <AppAlert />
                <AppTabs onChange={setSelectedTab} />
                <Box display="flex" flexGrow={1} minHeight="0">
                    <LocalModsPage show={selectedTab === "1"} />
                    <RemoteModsPage show={selectedTab === "2"} />
                    <UpdateModsPage show={selectedTab === "3"} />
                </Box>
            </TabContext>
        </BaseApp>
    );
};

export default MainApp;
