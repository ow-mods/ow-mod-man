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
import AlertBar from "./alerts/AlertBar";
import { ModalHandle } from "./modals/Modal";

startConsoleLogListen();

const App = () => {
    const [status, guiConfig, err] = hooks.getGuiConfig("GUI_CONFIG_RELOAD");
    useTheme(guiConfig?.theme ?? Theme.White, guiConfig?.rainbow ?? false);
    const owmlRef = useRef<ModalHandle>();

    if (import.meta.env.DEV) {
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        window._DEBUG_OPEN_SETUP = () => openOwmlSetup.current();
    }

    useEffect(() => {
        commands.initialSetup();
    }, []);

    useEffect(() => {
        getCurrent()
            .setTitle(TranslationMap[guiConfig?.language ?? "English"]["APP_TITLE"] ?? "Outer Wilds Mod Manager")
            .catch(console.warn);
    }, [guiConfig?.language]);

    const owmlCheck = useCallback(() => {
        commands.checkOWML().then((valid) => {
            if (!valid) owmlRef.current?.open();
        });
    }, []);

    useEffect(() => {
        if (status === "Done") {
            owmlCheck();
        }
    }, [status, owmlCheck]);

    useEffect(() => {
        let cancelled = false;
        listen("OWML_CONFIG_RELOAD", () => {
            if (cancelled) return;
            owmlCheck();
        });
        return () => {
            cancelled = true;
        };
    }, [owmlCheck]);

    if (status === "Loading" && guiConfig === null) {
        return <CenteredSpinner className="fill" />;
    } else if (status === "Error") {
        return <p className="fill center">Error loading configuration: {err!.toString()}</p>;
    } else {
        return (
            <TranslationContext.Provider value={guiConfig!.language}>
                <main className="container">
                    <OwmlSetupModal ref={owmlRef} />
                    <header>
                        <AlertBar />
                        <Nav />
                    </header>
                    <Tabs />
                </main>
            </TranslationContext.Provider>
        );
    }
};

export default App;
