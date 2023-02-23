import { hooks } from "@commands";
import { useTranslation } from "@hooks";
import LocalModRow from "./LocalModRow";

const LocalMods = () => {
    const [status, mods, err] = hooks.getLocalMods("LOCAL-REFRESH");

    const noMods = useTranslation("NO_MODS");

    if (status === "Loading" && mods === null) {
        return <div aria-busy className="mod-list center-loading"></div>;
    } else if (status === "Error") {
        return <div className="center-loading mod-list">{err!.toString()}</div>;
    } else {
        return (
            <div className="mod-list">
                {mods!.length === 0 && <p className="center-loading muted">{noMods}</p>}
                {mods!.map((m) => (
                    <LocalModRow key={m} uniqueName={m} />
                ))}
            </div>
        );
    }
};

export default LocalMods;
