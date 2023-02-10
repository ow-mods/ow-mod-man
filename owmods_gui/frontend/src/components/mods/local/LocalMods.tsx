import { useTauri } from "@hooks";
import LocalModRow from "./LocalModRow";

const LocalMods = () => {
    const [status, mods, err] = useTauri<string[], undefined>("LOCAL-REFRESH", "get_local_mods");

    if (status === "Loading" && mods === null) {
        return <div aria-busy className="mod-list center-loading"></div>;
    } else if (status === "Error") {
        return <div className="center-loading mod-list">{err!.toString()}</div>;
    } else {
        return (
            <div className="mod-list">
                {mods!.map((m) => (
                    <LocalModRow key={m} uniqueName={m} />
                ))}
            </div>
        );
    }
};

export default LocalMods;
