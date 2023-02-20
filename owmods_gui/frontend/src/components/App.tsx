import Nav from "@components/nav/Nav";
import Tabs from "@components/tabs/Tabs";
import { useGuiConfig } from "@hooks";
import { invoke } from "@tauri-apps/api";
import { getCurrent } from "@tauri-apps/api/window";
import { useCallback, useEffect, useRef } from "react";
import { TranslationContext, TranslationMap } from "@components/TranslationContext";
import ThemeMap from "../theme";

import rainbow from "@styles/rainbow.scss?inline";
import OwmlSetupModal from "./modals/OwmlSetupModal";
import { listen } from "@tauri-apps/api/event";
import { startLogListen } from "../logging";

startLogListen();

// Refresh once to get data
invoke("refresh_local_db").catch(() => console.warn("Can't fetch local DB"));
invoke("refresh_remote_db").catch(() => console.warn("Can't fetch remote DB"));

const App = () => {
    const [status, guiConfig, err] = useGuiConfig();
    const openOwmlSetup = useRef<() => void>(() => null);

    useEffect(() => {
        getCurrent()
            .setTitle(TranslationMap[guiConfig?.language ?? "English"]["APP_TITLE"])
            .catch(console.warn);
    }, [guiConfig?.language]);

    const owmlCheck = useCallback(() => {
        invoke("get_owml_config").catch(() => {
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
            invoke("get_owml_config").catch(() => {
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
