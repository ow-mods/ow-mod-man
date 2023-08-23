import ReactDOM from "react-dom/client";
import { commands } from "@commands";
import { ErrorBoundary } from "react-error-boundary";
import { basicFallbackRender, onError, simpleOnError } from "./errorHandling";
import { listen } from "@events";
import React from "react";

let port = 0;

const LogApp = React.lazy(() => import("@components/logs/LogApp"));

listen("gameStart", (inPort) => {
    if (port !== 0) return;

    port = inPort;

    ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
        <React.StrictMode>
            <ErrorBoundary fallbackRender={basicFallbackRender} onError={onError}>
                <LogApp port={port} />
            </ErrorBoundary>
        </React.StrictMode>
    );
});

commands.runGame().catch(simpleOnError);
