import LocalModRow from "@components/mods/local/LocalModRow";
import { LocalMod } from "src/types";

const LocalMods = () => {
    const mods: LocalMod[] = [
        {
            enabled: true,
            modPath: "C:/",
            manifest: {
                uniqueName: "Bwc9876.TimeSaver",
                author: "Bwc9876",
                name: "Time Saver",
                version: "0.0.1"
            }
        }
    ];

    return (
        <div className="mod-list">
            {mods.map((c) => (
                <LocalModRow key={c.manifest.uniqueName} {...c} />
            ))}
        </div>
    );
};

export default LocalMods;
