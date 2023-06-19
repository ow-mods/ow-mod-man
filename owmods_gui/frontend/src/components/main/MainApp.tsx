import theme from "../../theme";
import { Box, CircularProgress, CssBaseline, ThemeProvider, Typography } from "@mui/material";
import TopBar from "./top-bar/TopBar";
import { useEffect, useState } from "react";
import { TabContext } from "@mui/lab";
import AppTabs from "./top-bar/AppTabs";
import LocalModsPage from "./mods/local/LocalModsPage";
import RemoteModsPage from "./mods/remote/RemoteModsPage";
import UpdateModsPage from "./mods/updates/UpdateModsPage";
import { TranslationContext, TranslationMap } from "@components/common/TranslationContext";
import { commands, hooks } from "@commands";
import { ErrorRounded } from "@mui/icons-material";
import { getCurrent } from "@tauri-apps/api/window";

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
        <ThemeProvider theme={theme}>
            <CssBaseline>
                {status === "Error" || guiConfig === null ? (
                    <Box
                        width="100%"
                        height="100%"
                        display="flex"
                        alignItems="center"
                        justifyContent="center"
                    >
                        {status === "Loading" ? (
                            <CircularProgress color="neutral" />
                        ) : (
                            <Typography variant="h5" color="error">
                                <ErrorRounded /> Fatal Error <br /> {err?.toString()}
                            </Typography>
                        )}
                    </Box>
                ) : (
                    <TranslationContext.Provider value={guiConfig.language}>
                        <Box display="flex" flexDirection="column" height="100%">
                            <TabContext value={selectedTab}>
                                <TopBar />
                                <AppTabs onChange={setSelectedTab} />
                                <Box display="flex" flexGrow={1} minHeight="0">
                                    <LocalModsPage show={selectedTab === "1"} />
                                    <RemoteModsPage show={selectedTab === "2"} />
                                    <UpdateModsPage show={selectedTab === "3"} />
                                </Box>
                            </TabContext>
                        </Box>
                    </TranslationContext.Provider>
                )}
            </CssBaseline>
        </ThemeProvider>
    );
};

export default MainApp;
