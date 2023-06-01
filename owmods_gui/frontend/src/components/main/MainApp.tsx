import theme from "../../theme";
import { CssBaseline, ThemeProvider } from "@mui/material";
import TopBar from "./top-bar/TopBar";
import { useState } from "react";
import { TabContext } from "@mui/lab";
import AppTabs from "./top-bar/AppTabs";
import LocalModsPage from "./mods/local/LocalModsPage";

const MainApp = () => {
    const [selectedTab, setSelectedTab] = useState("1");

    return (
        <ThemeProvider theme={theme}>
            <TabContext value={selectedTab}>
                <CssBaseline>
                    <div
                        style={{
                            display: "flex",
                            flexDirection: "column",
                            height: "100%"
                        }}
                    >
                        <TopBar />
                        <AppTabs onChange={setSelectedTab} />
                        <div style={{ display: "flex", flexGrow: 1, minHeight: 0 }}>
                            <LocalModsPage show={selectedTab === "1"} />
                        </div>
                    </div>
                </CssBaseline>
            </TabContext>
        </ThemeProvider>
    );
};

export default MainApp;
