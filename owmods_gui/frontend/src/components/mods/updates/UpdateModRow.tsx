import { commands, hooks } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import Icon from "@components/common/Icon";
import { useTranslation } from "@hooks";
import { memo, useCallback, useMemo, useState } from "react";
import { BsArrowUp } from "react-icons/bs";
import ModActionButton from "../ModActionButton";
import ModHeader from "../ModHeader";

export interface UpdateModRowProps {
    uniqueName: string;
    onUpdate?: (updating: boolean) => void;
}

const UpdateModRow = memo(
    (props: UpdateModRowProps) => {
        const [remoteStatus, remoteMod, err1] = hooks.getRemoteMod("REMOTE-REFRESH", props);
        const [localStatus, localMod, err2] = hooks.getLocalMod("LOCAL-REFRESH", props);
        const [updating, setUpdating] = useState(false);

        const subtitleString = useMemo(
            () => `${localMod?.manifest.version ?? "v0"} 🡢 ${remoteMod?.version ?? "v0"}`,
            [remoteMod, localMod]
        );

        const updateLabel = useTranslation("UPDATE");

        const status = [remoteStatus, localStatus];

        const onUpdate = useCallback(() => {
            props.onUpdate?.(true);
            setUpdating(true);
            commands
                .updateMod({ uniqueName: props.uniqueName })
                .then(() => {
                    props.onUpdate?.(false);
                    setUpdating(false);
                    commands.refreshLocalDb().catch(console.warn);
                })
                .catch(console.error);
        }, [props.uniqueName]);

        if (status.includes("Loading") && remoteMod === null && localMod === null) {
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
                        {updating ? (
                            <CenteredSpinner />
                        ) : (
                            <ModActionButton onClick={onUpdate} ariaLabel={updateLabel}>
                                <Icon iconType={BsArrowUp} />
                            </ModActionButton>
                        )}
                    </ModHeader>
                </div>
            );
        }
    },
    (prev, next) => prev.uniqueName === next.uniqueName
);

export default UpdateModRow;
