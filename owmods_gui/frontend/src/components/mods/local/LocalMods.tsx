import { useTauri } from "@hooks";
import LocalModRow from "./LocalModRow";

const LocalMods = () => {
    const [status, mods, err] = useTauri<string[], undefined>("LOCAL-REFRESH", "get_local_mods");

    switch (status) {
        case "Loading":
            return <div aria-busy className="mod-list center-loading"></div>;
        case "Done":
            return (
                <div className="mod-list">
                    {mods!.map((m) => (
                        <LocalModRow key={m} uniqueName={m} />
                    ))}
                </div>
            );
        case "Error":
            return <div className="center-loading mod-list">{err!.toString()}</div>;
    }
};

export default LocalMods;
