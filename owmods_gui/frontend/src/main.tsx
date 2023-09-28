import React, { Suspense } from "react";
import ReactDOM from "react-dom/client";
import AppWrapper from "@components/main/AppWrapper";

const App = React.lazy(() => import("@components/main/MainApp"));

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <AppWrapper>
        <Suspense>
            <App />
        </Suspense>
    </AppWrapper>
);
