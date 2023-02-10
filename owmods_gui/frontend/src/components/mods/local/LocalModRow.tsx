import Icon from "@components/Icon";
import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { useTauri, useTranslations } from "@hooks";
import { LocalMod } from "@types";
import { memo } from "react";
import { FaFolder, FaTrash } from "react-icons/fa";

interface LocalModRowProps {
    uniqueName: string;
}

const LocalModRow = memo((props: LocalModRowProps) => {
    const [status, mod, err] = useTauri<LocalMod>("LOCAL-REFRESH", "get_local_mod", {
        uniqueName: props.uniqueName
    });

    const [showFolderTooltip, uninstallTooltip] = useTranslations(["SHOW_FOLDER", "UNINSTALL"]);

    //return <div className="mod-row local center-loading" aria-busy></div>;

    if (status === "Loading") {
        return <div className="mod-row local center-loading" aria-busy></div>;
    } else if (status === "Error") {
        return <div className="mod-row local center-loading">{err!.toString()}</div>;
    } else {
        const localMod = mod!;
        return (
            <div className="mod-row local">
                <ModHeader {...localMod.manifest}>
                    <ModActionButton ariaLabel={showFolderTooltip}>
                        <Icon iconType={FaFolder} />
                    </ModActionButton>
                    <ModActionButton ariaLabel={uninstallTooltip}>
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
            </div>
        );
    }
});

export default LocalModRow;
