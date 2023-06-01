import React from "react";
import ReactDOM from "react-dom/client";
//import App from "@components/logs/LogApp";
import { commands } from "@commands";
import { listen } from "@tauri-apps/api/event";

let port = 0;

listen("GAME-START", (e) => {
    if (port !== 0) return;

    port = e.payload as number;

    ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
        <React.StrictMode>{/* <App port={port} /> */}</React.StrictMode>
    );
}).then(() => commands.runGame().catch(console.warn));
