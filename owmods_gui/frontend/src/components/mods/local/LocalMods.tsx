import { useTauri } from "@hooks";
import LocalModRow from "./LocalModRow";

const LocalMods = () => {
    const [status, mods, err] = useTauri<string[]>("LOCAL-REFRESH", "get_local_mods");

    if (status === "Loading" && mods === null) {
        return <div aria-busy className="mod-list center-loading"></div>;
    } else if (status === "Error") {
        return <div className="center-loading mod-list">{err!.toString()}</div>;
    } else {
        return (
            <div className="mod-list">
                {mods!.length === 0 && (
                    <p className="center-loading muted">
                        No Mods Installed, Click &quot;Get Mods&quot; To Grab Some!
                    </p>
                )}
                {mods!.map((m) => (
                    <LocalModRow key={m} uniqueName={m} />
                ))}
            </div>
        );
    }
};

export default LocalMods;
