import Nav from "@components/nav/Nav";
import Tabs from "@components/tabs/Tabs";
import { getCurrent } from "@tauri-apps/api/window";
import { useCallback, useEffect, useRef } from "react";
import { TranslationContext, TranslationMap } from "@components/common/TranslationContext";

import OwmlSetupModal from "./modals/OwmlSetupModal";
import { listen } from "@tauri-apps/api/event";
import { startConsoleLogListen } from "../logging";
import { commands, hooks } from "@commands";
import { useTheme } from "@hooks";
import { Theme } from "@types";
import CenteredSpinner from "./common/CenteredSpinner";

startConsoleLogListen();

// Refresh once to get data
commands.refreshLocalDb().catch(() => console.warn("Can't fetch local DB"));
commands.refreshRemoteDb().catch(() => console.warn("Can't fetch remote DB"));

const App = () => {
    const [status, guiConfig, err] = hooks.getGuiConfig("GUI_CONFIG_RELOAD");
    useTheme(guiConfig?.theme ?? Theme.White, guiConfig?.rainbow ?? false);
    const openOwmlSetup = useRef<() => void>(() => null);

    if (import.meta.env.DEV) {
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        window._DEBUG_OPEN_SETUP = () => openOwmlSetup.current();
    }

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

    if (status === "Loading" && guiConfig === null) {
        return <CenteredSpinner className="fill" />;
    } else if (status === "Error") {
        return <p className="fill center">Error loading configuration: {err!.toString()}</p>;
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
