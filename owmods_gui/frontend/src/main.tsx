import React from "react";
import ReactDOM from "react-dom/client";
import App from "@components/main/MainApp";
import { ErrorBoundary } from "react-error-boundary";
import { basicFallbackRender, onError } from "@components/common/StyledErrorBoundary";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <ErrorBoundary fallbackRender={basicFallbackRender} onError={onError}>
            <App />
        </ErrorBoundary>
    </React.StrictMode>
);
