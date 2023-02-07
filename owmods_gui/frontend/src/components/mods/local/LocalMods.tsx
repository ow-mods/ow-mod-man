import { useTauri } from "@hooks";
import LocalModRow from "./LocalModRow";

const LocalMods = () => {
    const [status, mods, err] = useTauri<string[], undefined>("LOCAL-REFRESH", "get_local_mods");

    switch (status) {
        case "Loading":
            return <p>Loading</p>;
        case "Done":
            return (
                <div className="mod-list">
                    {mods!.map((m) => (
                        <LocalModRow key={m} uniqueName={m} />
                    ))}
                </div>
            );
        case "Error":
            return <div>{err!}</div>;
    }
};

export default LocalMods;
