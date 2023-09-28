import React from "react";
import ReactDOM from "react-dom/client";
import AppWrapper from "@components/main/AppWrapper";

const App = React.lazy(() => import("@components/main/MainApp"));

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <AppWrapper>
        <App />
    </AppWrapper>
);
