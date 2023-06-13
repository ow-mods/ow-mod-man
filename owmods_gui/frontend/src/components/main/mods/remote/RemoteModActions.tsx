import { DownloadRounded, DescriptionRounded, ScienceRounded } from "@mui/icons-material";
import { Box, CircularProgress } from "@mui/material";
import { memo, useRef } from "react";
import ModActionIcon from "../ModActionIcon";
import ModActionOverflow, { ModActionOverflowItem } from "../ModActionOverflow";
import { useGetTranslation } from "@hooks";

export interface RemoteModActionsProps {
    uniqueName: string;
    busy: boolean;
    showPrerelease: boolean;
    prereleaseLabel: string;
    onInstall: () => void;
    onReadme: () => void;
    onPrerelease: () => void;
}

const RemoteModActions = memo(function RemoteModToolbar(props: RemoteModActionsProps) {
    const getTranslation = useGetTranslation();
    const overflowRef = useRef<{ onClose: () => void }>();

    return (
        <>
            {props.busy ? (
                <Box display="flex" alignItems="center">
                    <CircularProgress color="inherit" size={22} />
                </Box>
            ) : (
                <ModActionIcon
                    onClick={props.onInstall}
                    label={getTranslation("INSTALL")}
                    icon={<DownloadRounded />}
                />
            )}
            <ModActionOverflow id={`remote-${props.uniqueName}`} ref={overflowRef}>
                <ModActionOverflowItem
                    label={getTranslation("OPEN_README")}
                    icon={<DescriptionRounded />}
                    onClick={props.onReadme}
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
