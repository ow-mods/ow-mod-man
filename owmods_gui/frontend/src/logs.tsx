import ReactDOM from "react-dom/client";
import { commands } from "@commands";
import { listen } from "@tauri-apps/api/event";
import LogApp from "@components/logs/LogApp";
import { ErrorBoundary } from "react-error-boundary";
import { fallbackRender, onError } from "./main";

let port = 0;

listen("GAME-START", (e) => {
    if (port !== 0) return;

    port = e.payload as number;

    ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
        <ErrorBoundary fallbackRender={fallbackRender} onError={onError}>
            <LogApp port={port} />
        </ErrorBoundary>
    );
}).then(() => commands.runGame().catch(console.warn));
