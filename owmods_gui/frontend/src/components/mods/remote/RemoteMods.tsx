import { useTauri } from "@hooks";
import RemoteModRow from "./RemoteModRow";

const RemoteMods = () => {
    const [status, mods, err] = useTauri<string[], undefined>("REMOTE-REFRESH", "get_remote_mods");

    if (status === "Loading") {
        return <p>Loading...</p>;
    } else if (status === "Error") {
        return <p>{err}</p>;
    } else {
        const remote_mods = mods!;
        return (
            <div className="mod-list">
                {remote_mods.map((m) => (
                    <RemoteModRow key={m} uniqueName={m} />
                ))}
            </div>
        );
    }
};

export default RemoteMods;
