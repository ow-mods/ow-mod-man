import React from "react";
import ReactDOM from "react-dom/client";
import App from "@components/main/MainApp";
import { ErrorBoundary } from "react-error-boundary";
import { onError } from "@components/common/StyledErrorBoundary";

export const fallbackRender = ({ error }: { error: unknown }) => (
    <p
        style={{ color: "rgb(255, 83, 83)", marginTop: "50vh", textAlign: "center" }}
    >{`Fatal Error: ${error}`}</p>
);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <ErrorBoundary fallbackRender={fallbackRender} onError={onError}>
            <App />
        </ErrorBoundary>
    </React.StrictMode>
);
