import React, { Suspense } from "react";
import ReactDOM from "react-dom/client";
import { ErrorBoundary } from "react-error-boundary";
import { basicFallbackRender, onError } from "./errorHandling";

// Prevent ctrl+p from opening the print dialog
document.addEventListener("keydown", (e) => {
    if (e.ctrlKey && e.key === "p") {
        e.preventDefault();
    }
});

const App = React.lazy(() => import("@components/main/MainApp"));

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <ErrorBoundary fallbackRender={basicFallbackRender} onError={onError}>
            <Suspense>
                <App />
            </Suspense>
        </ErrorBoundary>
    </React.StrictMode>
);
