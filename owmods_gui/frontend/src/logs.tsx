import React from "react";
import ReactDOM from "react-dom/client";
import App from "@components/logs/LogApp";
import "@styles/style.scss";
import { commands } from "@commands";

commands.logsAreActive().then((res) => {
    if (!res) {
        commands.runGame().catch(console.warn);
    }
    ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
        <React.StrictMode>
            <App />
        </React.StrictMode>
    );
});
