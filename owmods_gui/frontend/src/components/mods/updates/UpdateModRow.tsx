import Icon from "@components/Icon";
import { useTauri, useTranslation } from "@hooks";
import { invoke } from "@tauri-apps/api";
import { LocalMod, RemoteMod } from "@types";
import { memo, useCallback, useMemo, useState } from "react";
import { FaArrowAltCircleUp } from "react-icons/fa";
import ModActionButton from "../ModActionButton";
import ModHeader from "../ModHeader";

const UpdateModRow = memo((props: { uniqueName: string }) => {
    const [remoteStatus, remoteMod, err1] = useTauri<RemoteMod>(
        "REMOTE-REFRESH",
        "get_remote_mod",
        props
    );
    const [localStatus, localMod, err2] = useTauri<LocalMod>(
        "LOCAL-REFRESH",
        "get_local_mod",
        props
    );
    const [updating, setUpdating] = useState(false);

    const subtitleString = useMemo(
        () => `${localMod?.manifest.version ?? "v0"} 🡢 ${remoteMod?.version ?? "v0"}`,
        [remoteMod, localMod]
    );

    const updateLabel = useTranslation("UPDATE");

    const status = [remoteStatus, localStatus];

    const onUpdate = useCallback(() => {
        setUpdating(true);
        invoke("update_mod", { uniqueName: props.uniqueName })
            .then(() => {
                setUpdating(false);
                invoke("refresh_local_db").catch(console.warn);
            })
            .catch(console.error);
    }, [props.uniqueName]);

    if (status.includes("Loading") && remoteMod === null && localMod === null) {
        return <div className="mod-row center-loading" aria-busy></div>;
    } else if (status.includes("Error")) {
        return (
            <p className="mod-row center-loading">
                {err1?.toString() ?? ""} {err2?.toString() ?? ""}
            </p>
        );
    } else {
        return (
            <div className="mod-row">
                <ModHeader {...remoteMod!} subtitle={subtitleString}>
                    {updating ? (
                        <div className="center-loading" aria-busy></div>
                    ) : (
                        <ModActionButton onClick={onUpdate} ariaLabel={updateLabel}>
                            <Icon iconType={FaArrowAltCircleUp} />
                        </ModActionButton>
                    )}
                </ModHeader>
            </div>
        );
    }
});

export default UpdateModRow;
