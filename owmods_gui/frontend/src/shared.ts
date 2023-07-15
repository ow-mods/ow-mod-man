// Shared module between all windows that does common setup outside of React

import "@fontsource/roboto/300.css";
import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";

// In dev the context menu is useful for reloading and inspect element
if (!import.meta.env.DEV) {
    document.oncontextmenu = (e) => {
        e.preventDefault();
        return false;
    };
}
