import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { FaArrowDown, FaGlobe } from "react-icons/fa";

export interface RemoteModProps {
    name: string;
    authors: string;
    description: string;
    downloads: number;
}

export default (props: RemoteModProps) => {
    return (
        <details>
            <ModHeader {...props}>
                <small>{props.downloads}</small>
                <ModActionButton ariaLabel="Install With Dependencies">
                    <FaArrowDown />
                </ModActionButton>
                <ModActionButton ariaLabel="View On Website">
                    <FaGlobe />
                </ModActionButton>
            </ModHeader>
            <small>{props.description}</small>
        </details>
    );
};
