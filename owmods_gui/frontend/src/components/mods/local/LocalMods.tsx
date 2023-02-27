import { hooks } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import { useTranslation } from "@hooks";
import { memo } from "react";
import LocalModRow from "./LocalModRow";

const LocalMods = memo(() => {
    const [status, mods, err] = hooks.getLocalMods("LOCAL-REFRESH");

    const noMods = useTranslation("NO_MODS");

    if (status === "Loading" && mods === null) {
        return <CenteredSpinner className="mod-list" />;
    } else if (status === "Error") {
        return <div className="center mod-list">{err!.toString()}</div>;
    } else {
        return (
            <div className="mod-list">
                {mods!.length === 0 && <p className="center muted">{noMods}</p>}
                {mods!.map((m) => (
                    <LocalModRow key={m} uniqueName={m} />
                ))}
            </div>
        );
    }
});

export default LocalMods;
