import Nav from "@components/nav/Nav";
import Tabs from "@components/tabs/Tabs";
import { useGuiConfig } from "@hooks";
import { invoke } from "@tauri-apps/api";
import { getCurrent } from "@tauri-apps/api/window";
import { useEffect } from "react";
import { TranslationContext, TranslationMap } from "@components/TranslationContext";
import ThemeMap from "../theme";

import rainbow from "@styles/rainbow.scss?inline";

// Refresh once to get data
invoke("refresh_local_db").catch(() => console.warn("Can't fetch local DB"));
invoke("refresh_remote_db").catch(() => console.warn("Can't fetch remote DB"));

const App = () => {
    const [status, guiConfig, err] = useGuiConfig();

    useEffect(() => {
        getCurrent()
            .setTitle(TranslationMap[guiConfig?.language ?? "English"]["APP_TITLE"])
            .catch(console.warn);
    }, [guiConfig?.language]);

    useEffect(() => {
        let newTheme = ThemeMap[guiConfig?.theme ?? "White"];
        if (guiConfig?.rainbow /*|| (new Date()).getMonth() === 5*/) {
            newTheme += rainbow;
        }
        document.getElementById("currentTheme")!.textContent = newTheme;
    }, [guiConfig?.theme, guiConfig?.rainbow]);

    if (status === "Loading" && guiConfig === null) {
        return <div className="fill center-loading" aria-busy></div>;
    } else if (status === "Error") {
        return (
            <p className="fill center-loading">Error loading configuration: {err!.toString()}</p>
        );
    } else {
        return (
            <TranslationContext.Provider value={guiConfig!.language}>
                <main className="container">
                    <header>
                        <Nav />
                    </header>
                    <Tabs />
                </main>
            </TranslationContext.Provider>
        );
    }
};

export default App;
