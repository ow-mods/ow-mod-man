import { DownloadRounded, DescriptionRounded, GitHub } from "@mui/icons-material";
import { memo, useRef } from "react";
import ModActionOverflow, { ModActionOverflowItem } from "../ModActionOverflow";
import { useGetTranslation } from "@hooks";
import ModDownloadIcon from "../ModDownloadIcon";

export interface RemoteModActionsProps {
    uniqueName: string;
    busy: boolean;
    showPrerelease: boolean;
    prereleaseLabel: string;
    onInstall: () => void;
    onReadme: () => void;
    onGithub: () => void;
    onPrerelease: () => void;
}

const RemoteModActions = memo(function RemoteModToolbar(props: RemoteModActionsProps) {
    const getTranslation = useGetTranslation();
    const overflowRef = useRef<{ onClose: () => void }>({ onClose: () => {} });

    const onClose = () => {
        overflowRef.current?.onClose?.();
    };

    return (
        <>
            <ModDownloadIcon
                icon={<DownloadRounded />}
                tooltip={getTranslation("INSTALL")}
                onClick={props.onInstall}
                uniqueName={props.uniqueName}
            />
            <ModActionOverflow
                showPrerelease={props.showPrerelease}
                prereleaseLabel={props.prereleaseLabel}
                onPrerelease={props.onPrerelease}
                tabId="remote"
                uniqueName={props.uniqueName}
                busy={props.busy}
                ref={overflowRef}
            >
                <ModActionOverflowItem
                    label={getTranslation("OPEN_README")}
                    icon={<DescriptionRounded />}
                    onClick={props.onReadme}
                    onClose={onClose}
                />
                <ModActionOverflowItem
                    label={getTranslation("OPEN_GITHUB")}
                    icon={<GitHub />}
                    onClick={props.onGithub}
                    onClose={onClose}
                />
            </ModActionOverflow>
        </>
    );
});

export default RemoteModActions;
