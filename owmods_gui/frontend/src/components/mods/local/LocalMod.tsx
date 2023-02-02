import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { useState } from "react";
import { FaFolder, FaTrash } from "react-icons/fa";

export interface LocalModProps {
    name: string;
    authors: string;
    description: string;
    enabled: boolean;
}

export default (props: LocalModProps) => {
    // Temp state for testing stuff
    const [enabled, setEnabled] = useState(props.enabled);

    return (
        <details>
            <ModHeader {...props}>
                <ModActionButton ariaLabel="Show Folder">
                    <FaFolder />
                </ModActionButton>
                <ModActionButton ariaLabel="Uninstall Mod">
                    <FaTrash />
                </ModActionButton>
                <input
                    className="mod-toggle"
                    onClick={() => setEnabled(!enabled)}
                    type="checkbox"
                    aria-label="enabled"
                    role="switch"
                    value={enabled.toString()}
                />
            </ModHeader>
            <small>{props.description}</small>
        </details>
    );
};
