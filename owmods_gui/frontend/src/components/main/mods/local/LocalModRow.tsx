import { commands, hooks } from "@commands";
import ModRow from "../ModRow";
import { LocalMod, UnsafeLocalMod } from "@types";
import { useMemo } from "react";
import { Checkbox } from "@mui/material";
import ModActionIcon from "../ModActionIcon";
import { useGetTranslation } from "@hooks";
import {
    DeleteRounded,
    DescriptionRounded,
    FolderRounded,
    UpdateRounded
} from "@mui/icons-material";

commands.refreshRemoteDb().then(() => commands.refreshLocalDb());

export interface LocalModRowProps {
    uniqueName: string;
}

const safeOrNull = (mod: UnsafeLocalMod | null) => {
    if (mod === null) return null;
    if (mod.loadState === "invalid") {
        return null;
    } else {
        return mod.mod as LocalMod;
    }
};

const LocalModRow = (props: LocalModRowProps) => {
    const getTranslation = useGetTranslation();

    const local = hooks.getLocalMod("LOCAL_REFRESH", { ...props })[1];
    const remote = hooks.getRemoteMod("REMOTE_REFRESH", { ...props })[1];

    const name = useMemo(
        () => remote?.name ?? safeOrNull(local)?.manifest.name ?? "",
        [local, remote]
    );
    const author = useMemo(
        () => remote?.authorDisplay ?? remote?.author ?? safeOrNull(local)?.manifest.author ?? "",
        [local, remote]
    );

    const description = remote?.description;

    const version = useMemo(() => safeOrNull(local)?.manifest.version ?? "--", [local]);

    const enabled = safeOrNull(local)?.enabled ?? false;

    const outdated = useMemo(
        () =>
            safeOrNull(local)?.errors.filter((e) => e.errorType === "Outdated").length !== 0 ??
            false,
        [local]
    );

    const onReadme = () => commands.openModReadme({ uniqueName: props.uniqueName });

    return (
        <ModRow
            uniqueName={props.uniqueName}
            name={name}
            author={author}
            version={version}
            isOutdated={outdated}
            description={description}
            downloads={remote?.downloadCount.toString() ?? "--"}
            overflow={[
                {
                    icon: <FolderRounded />,
                    label: getTranslation("SHOW_FOLDER")
                },
                {
                    icon: <DeleteRounded />,
                    label: getTranslation("UNINSTALL")
                }
            ]}
        >
            <Checkbox color="default" checked={enabled} />
            {outdated && (
                <ModActionIcon
                    icon={<UpdateRounded color="secondary" />}
                    label={getTranslation("UPDATE")}
                />
            )}
            <ModActionIcon
                onClick={onReadme}
                label={getTranslation("OPEN_WEBSITE")}
                icon={<DescriptionRounded />}
            />
        </ModRow>
    );
};

export default LocalModRow;
