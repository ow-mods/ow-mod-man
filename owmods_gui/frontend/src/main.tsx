import React from "react";
import ReactDOM from "react-dom/client";
import App from "@components/App";
import "@styles/style.scss";

import { IconContext } from "react-icons";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);
