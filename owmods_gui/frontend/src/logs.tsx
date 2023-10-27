import ReactDOM from "react-dom/client";
import { commands } from "@commands";
import { simpleOnError } from "./errorHandling";
import { listen } from "@events";
import React from "react";
import AppWrapper from "@components/AppWrapper";

let port = 0;

const LogApp = React.lazy(() => import("@components/logs/LogApp"));

listen("gameStart", (inPort) => {
    if (port !== 0) return;

    port = inPort;

    ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
        <AppWrapper>
            <LogApp port={port} />
        </AppWrapper>
    );
});

commands.runGame().catch(simpleOnError);
