import Nav from "@components/nav/Nav";
import Tabs from "@components/tabs/Tabs";
import { getCurrent } from "@tauri-apps/api/window";
import { useCallback, useEffect, useRef } from "react";
import { TranslationContext, TranslationMap } from "@components/TranslationContext";
import ThemeMap from "../theme";

import rainbow from "@styles/rainbow.scss?inline";
import OwmlSetupModal from "./modals/OwmlSetupModal";
import { listen } from "@tauri-apps/api/event";
import { startLogListen } from "../logging";
import { commands, hooks } from "@commands";

startLogListen();

// Refresh once to get data
commands.refreshLocalDb().catch(() => console.warn("Can't fetch local DB"));
commands.refreshRemoteDb().catch(() => console.warn("Can't fetch remote DB"));

const App = () => {
    const [status, guiConfig, err] = hooks.getGuiConfig("GUI_CONFIG_RELOAD");
    const openOwmlSetup = useRef<() => void>(() => null);

    useEffect(() => {
        getCurrent()
            .setTitle(TranslationMap[guiConfig?.language ?? "English"]["APP_TITLE"])
            .catch(console.warn);
    }, [guiConfig?.language]);

    const owmlCheck = useCallback(() => {
        commands.getOwmlConfig().catch(() => {
            openOwmlSetup.current();
        });
    }, [openOwmlSetup]);

    useEffect(() => {
        if (status === "Done") {
            owmlCheck();
        }
    }, [status]);

    useEffect(() => {
        let cancelled = false;
        listen("OWML_CONFIG_RELOAD", () => {
            if (cancelled) return;
            commands.getOwmlConfig().catch(() => {
                openOwmlSetup.current();
            });
        });
        return () => {
            cancelled = true;
        };
    }, []);

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
                    <OwmlSetupModal open={openOwmlSetup} />
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
