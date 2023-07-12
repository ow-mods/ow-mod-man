import ReactDOM from "react-dom/client";
import { commands } from "@commands";
import LogApp from "@components/logs/LogApp";
import { ErrorBoundary } from "react-error-boundary";
import { basicFallbackRender, onError } from "@components/common/StyledErrorBoundary";
import { listen } from "@events";

let port = 0;

listen("gameStart", (inPort) => {
    if (port !== 0) return;

    port = inPort;

    ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
        <ErrorBoundary fallbackRender={basicFallbackRender} onError={onError}>
            <LogApp port={port} />
        </ErrorBoundary>
    );
}).then(() => commands.runGame().catch(commands.logError));
