import Icon from "@components/Icon";
import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { memo } from "react";
import { FaFolder, FaTrash } from "react-icons/fa";
import { LocalMod } from "src/types";

const isEqual = (prev: LocalMod, next: LocalMod) =>
    prev.manifest.uniqueName === next.manifest.uniqueName;

const LocalModRow = memo((props: LocalMod) => {
    return (
        <details>
            <ModHeader {...props.manifest}>
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
                    value={props.enabled.toString()}
                />
            </ModHeader>
            Description Not Available
        </details>
    );
}, isEqual);

export default LocalModRow;
