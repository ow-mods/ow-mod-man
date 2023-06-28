import React from "react";
import ReactDOM from "react-dom/client";
import App from "@components/main/MainApp";
import { ErrorBoundary } from "react-error-boundary";
import { commands } from "@commands";

export const fallbackRender = ({ error }: { error: unknown }) => (
    <p
        style={{ color: "rgb(255, 83, 83)", marginTop: "50vh", textAlign: "center" }}
    >{`Fatal Error: ${error}`}</p>
);

export const onError = (err: unknown, info: { componentStack: string }) => {
    commands.logError({
        err: `${err?.toString() ?? "null"}\nAt: ${info.componentStack}`
    });
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <ErrorBoundary fallbackRender={fallbackRender} onError={onError}>
            <App />
        </ErrorBoundary>
    </React.StrictMode>
);
