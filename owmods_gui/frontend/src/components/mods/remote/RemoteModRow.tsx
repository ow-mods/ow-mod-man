import Icon from "@components/Icon";
import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { useTauri } from "@hooks";
import { memo } from "react";
import { FaArrowDown, FaGlobe } from "react-icons/fa";
import { RemoteMod } from "src/types";

const RemoteModRow = memo((props: { uniqueName: string }) => {
    const [status, mod, err] = useTauri<RemoteMod, { uniqueName: string }>(
        "REMOTE-REFRESH",
        "get_remote_mod",
        { uniqueName: props.uniqueName }
    );

    if (status === "Loading") {
        return <p>Loading...</p>;
    } else if (status === "Error") {
        return <p>{err}</p>;
    } else {
        const remote_mod = mod!;
        return (
            <details>
                <ModHeader {...remote_mod}>
                    <small>{remote_mod.downloadCount}</small>
                    <ModActionButton ariaLabel="Install With Dependencies">
                        <Icon iconType={FaArrowDown} />
                    </ModActionButton>
                    <ModActionButton ariaLabel="View On Website">
                        <Icon iconType={FaGlobe} />
                    </ModActionButton>
                </ModHeader>
                <small>{remote_mod.description}</small>
            </details>
        );
    }
});

export default RemoteModRow;
