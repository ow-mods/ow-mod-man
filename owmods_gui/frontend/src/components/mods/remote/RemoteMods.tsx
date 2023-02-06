import { RemoteMod } from "src/types";
import RemoteModRow from "./RemoteModRow";

const RemoteMods = () => {
    const mods: RemoteMod[] = [
        {
            uniqueName: "Bwc9876.TimeSaver",
            downloadUrl: "google",
            downloadCount: 50,
            description: "abooga",
            repo: "ff",
            author: "Bwc9876",
            name: "Time Saver",
            version: "0.0.1"
        }
    ];

    return (
        <div className="mod-list">
            {mods.map((c) => (
                <RemoteModRow {...c} key={c.uniqueName} />
            ))}
        </div>
    );
};

export default RemoteMods;
