import { hooks } from "@commands";
import { memo } from "react";

const AlertBar = memo(function AlertBar() {
    const [status, alert, err] = hooks.getAlert("CONFIG_RELOAD");

    if (status === "Loading") {
        return <></>;
    } else if (status === "Error") {
        console.error(err);
        return <></>;
    } else {
        if (alert!.enabled) {
            return <span className="alert-row">{alert!.message}</span>;
        } else {
            return <></>;
        }
    }
});

export default AlertBar;
