import { commands, hooks } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import Icon from "@components/common/Icon";
import { useGetTranslation } from "@hooks";
import { memo, useCallback, useMemo, useState } from "react";
import { BsArrowUp } from "react-icons/bs";
import ModActionButton from "../ModActionButton";
import ModHeader from "../ModHeader";
import { LocalMod } from "@types";

export interface UpdateModRowProps {
    uniqueName: string;
    parentUpdating?: boolean;
    onUpdate?: (updating: boolean) => void;
}

const UpdateModRow = memo(
    function UpdateModRow({ uniqueName, parentUpdating, onUpdate }: UpdateModRowProps) {
        const getTranslation = useGetTranslation();
        const [remoteStatus, remoteMod, err1] = hooks.getRemoteMod("REMOTE-REFRESH", {
            uniqueName
        });
        const [localStatus, unsafeLocalMod, err2] = hooks.getLocalMod("LOCAL-REFRESH", {
            uniqueName
        });
        const [updating, setUpdating] = useState(false);

        // Assertion is safe bc we're only iterating over valid mods
        const localMod = unsafeLocalMod?.mod as LocalMod | null;

        const subtitleString = useMemo(
            () => `${localMod?.manifest.version ?? "v0"} 🡢 ${remoteMod?.version ?? "v0"}`,
            [remoteMod, localMod]
        );

        const status = [remoteStatus, localStatus];

        const onModUpdate = useCallback(() => {
            if (parentUpdating) return;
            onUpdate?.(true);
            setUpdating(true);
            commands
                .updateMod({ uniqueName })
                .then(() => {
                    onUpdate?.(false);
                    setUpdating(false);
                    commands.refreshLocalDb().catch(console.warn);
                })
                .catch(console.error);
        }, [uniqueName, onUpdate, parentUpdating]);

        if (status.includes("Loading") && (remoteMod === null || localMod === null)) {
            return <CenteredSpinner className="mod-row" />;
        } else if (status.includes("Error")) {
            return (
                <p className="mod-row center">
                    {err1?.toString() ?? ""} {err2?.toString() ?? ""}
                </p>
            );
        } else {
            return (
                <div className="mod-row">
                    <ModHeader {...remoteMod!} subtitle={subtitleString}>
                        {updating || parentUpdating ? (
                            <CenteredSpinner />
                        ) : (
                            <ModActionButton onClick={onModUpdate} ariaLabel={getTranslation("UPDATE")}>
                                <Icon iconType={BsArrowUp} />
                            </ModActionButton>
                        )}
                    </ModHeader>
                </div>
            );
        }
    },
    (prev, next) =>
        prev.uniqueName === next.uniqueName && prev.parentUpdating === next.parentUpdating
);

export default UpdateModRow;
