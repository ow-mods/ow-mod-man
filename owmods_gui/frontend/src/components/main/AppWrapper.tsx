import React, { ReactNode, Suspense } from "react";
import { ErrorBoundary } from "react-error-boundary";
import { basicFallbackRender, onError } from "src/errorHandling";

// Prevent ctrl+p from opening the print dialog
document.addEventListener("keydown", (e) => {
    if (e.ctrlKey && e.key === "p") {
        e.preventDefault();
    }
});

const AppWrapper = (props: { children: ReactNode }) => (
    <React.StrictMode>
        <ErrorBoundary fallbackRender={basicFallbackRender} onError={onError}>
            <Suspense>{props.children}</Suspense>
        </ErrorBoundary>
    </React.StrictMode>
);

export default AppWrapper;
