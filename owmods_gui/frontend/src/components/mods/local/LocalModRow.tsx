import Icon from "@components/Icon";
import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { useTauri } from "@hooks";
import { LocalMod } from "@types";
import { memo } from "react";
import { FaFolder, FaTrash } from "react-icons/fa";

const LocalModRow = memo((props: { uniqueName: string }) => {
    const [status, mod, err] = useTauri<LocalMod, { uniqueName: string }>(
        "LOCAL-REFRESH",
        "get_local_mod",
        { uniqueName: props.uniqueName }
    );

    if (status === "Loading") {
        return <p>Loading</p>;
    } else if (status === "Error") {
        return <p>{err!}</p>;
    } else {
        const localMod = mod!;
        return (
            <details>
                <ModHeader {...localMod.manifest}>
                    <ModActionButton ariaLabel="Show Folder">
                        <Icon iconType={FaFolder} />
                    </ModActionButton>
                    <ModActionButton ariaLabel="Uninstall Mod">
                        <Icon iconType={FaTrash} />
                    </ModActionButton>
                    <input
                        className="mod-toggle"
                        type="checkbox"
                        aria-label="enabled"
                        role="switch"
                        checked={localMod.enabled}
                    />
                </ModHeader>
                Description Not Available
            </details>
        );
    }
});

export default LocalModRow;
