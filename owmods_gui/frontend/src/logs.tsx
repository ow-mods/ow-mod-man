import React from "react";
import ReactDOM from "react-dom/client";
import App from "@components/logs/LogApp";
import "@styles/style.scss";
import { commands } from "@commands";
import { listen } from "@tauri-apps/api/event";
import { startConsoleLogListen } from "./logging";
import { getCurrent } from "@tauri-apps/api/window";

const thisWindow = getCurrent();

let port = 0;

listen("GAME-START", (e) => {
    if (port !== 0) return;

    port = e.payload as number;
    let unlisten: () => void = () => null;

    thisWindow
        .onCloseRequested((e) => {
            e.preventDefault();
            commands
                .stopLogging({ port })
                .catch(console.warn)
                .finally(() => {
                    unlisten();
                    thisWindow.close();
                });
        })
        .then((u) => {
            unlisten = u;
        });

    ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
        <React.StrictMode>
            <App port={port} />
        </React.StrictMode>
    );
}).then(() => commands.runGame().catch(console.warn));

startConsoleLogListen();
