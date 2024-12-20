import { DownloadRounded, DescriptionRounded, GitHub, ScienceRounded } from "@mui/icons-material";
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

    return (
        <>
            <ModDownloadIcon
                icon={<DownloadRounded />}
                tooltip={getTranslation("INSTALL")}
                onClick={props.onInstall}
                uniqueName={props.uniqueName}
            />
            <ModActionOverflow tabId="remote" uniqueName={props.uniqueName} ref={overflowRef}>
                <ModActionOverflowItem
                    label={getTranslation("OPEN_README")}
                    icon={<DescriptionRounded />}
                    onClick={props.onReadme}
                    onClose={overflowRef.current?.onClose}
                />
                <ModActionOverflowItem
                    label={getTranslation("OPEN_GITHUB")}
                    icon={<GitHub />}
                    onClick={props.onGithub}
                    onClose={overflowRef.current?.onClose}
                />
                {props.showPrerelease && (
                    <ModActionOverflowItem
                        label={props.prereleaseLabel}
                        icon={<ScienceRounded />}
                        onClick={props.onPrerelease}
                        disabled={props.busy ?? false}
                        onClose={overflowRef.current?.onClose}
                    />
                )}
            </ModActionOverflow>
        </>
    );
});

export default RemoteModActions;
