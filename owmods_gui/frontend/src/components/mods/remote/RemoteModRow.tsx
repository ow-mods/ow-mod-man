import Icon from "@components/Icon";
import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { memo } from "react";
import { FaArrowDown, FaGlobe } from "react-icons/fa";
import { RemoteMod } from "src/types";

const RemoteModRow = memo((props: RemoteMod) => {
    return (
        <details>
            <ModHeader {...props}>
                <small>{props.downloadCount}</small>
                <ModActionButton ariaLabel="Install With Dependencies">
                    <Icon iconType={FaArrowDown} />
                </ModActionButton>
                <ModActionButton ariaLabel="View On Website">
                    <Icon iconType={FaGlobe} />
                </ModActionButton>
            </ModHeader>
            <small>{props.description}</small>
        </details>
    );
});

export default RemoteModRow;
