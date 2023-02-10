import Icon from "@components/Icon";
import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { useTauri, useTranslations } from "@hooks";
import { invoke } from "@tauri-apps/api";
import { CSSProperties, memo, useCallback, useState } from "react";
import { FaArrowDown, FaFileAlt } from "react-icons/fa";
import { RemoteMod } from "src/types";

// Stolen from mods website, Rai will never catch me!
const magnitudeMap = [
    { value: 1, symbol: "" },
    { value: 1e3, symbol: "k" },
    { value: 1e6, symbol: "M" },
    { value: 1e9, symbol: "G" },
    { value: 1e12, symbol: "T" },
    { value: 1e15, symbol: "P" },
    { value: 1e18, symbol: "E" }
];

const numberFormatRegex = /\.0+$|(\.[0-9]*[1-9])0+$/;

export const formatNumber = (value: number, digits = 1) => {
    const magnitude = magnitudeMap
        .slice()
        .reverse()
        .find((item) => {
            return value >= item.value;
        });
    return magnitude
        ? (value / magnitude.value).toFixed(digits).replace(numberFormatRegex, "$1") +
              magnitude.symbol
        : "0";
};

export interface RemoteModRowProps {
    uniqueName: string;
    style?: CSSProperties;
}

const RemoteModRow = memo((props: RemoteModRowProps) => {
    const [status, mod, err] = useTauri<RemoteMod>("REMOTE-REFRESH", "get_remote_mod", {
        uniqueName: props.uniqueName
    });

    const [downloading, setDownloading] = useState(false);

    const [noDescription, installTooltip, websiteTooltip] = useTranslations([
        "NO_DESCRIPTION",
        "INSTALL",
        "OPEN_README"
    ]);

    const onInstall = useCallback(() => {
        setDownloading(true);
        invoke("install_mod", { uniqueName: props.uniqueName })
            .then(() => {
                setDownloading(false);
                invoke("refresh_local_db").catch(console.error);
            })
            .catch(console.error);
    }, [props.uniqueName]);

    const onReadme = useCallback(() => {
        invoke("open_mod_readme", { uniqueName: props.uniqueName }).catch(console.warn);
    }, [props.uniqueName]);

    if (status === "Loading") {
        return <div className="mod-row center-loading" aria-busy style={props.style}></div>;
    } else if (status === "Error") {
        return (
            <p className="mod-row center-loading" style={props.style}>
                {err!.toString()}
            </p>
        );
    } else {
        const remote_mod = mod!;
        let desc = remote_mod.description ?? noDescription;
        if (desc.trim() === "") desc = noDescription;
        return (
            <div style={props.style} className="mod-row">
                <ModHeader {...remote_mod} author={remote_mod.authorDisplay ?? remote_mod.author}>
                    <small>{formatNumber(remote_mod.downloadCount)}</small>
                    {downloading ? (
                        <div className="center-loading" aria-busy></div>
                    ) : (
                        <ModActionButton onClick={onInstall} ariaLabel={installTooltip}>
                            <Icon iconType={FaArrowDown} />
                        </ModActionButton>
                    )}
                    <ModActionButton onClick={onReadme} ariaLabel={websiteTooltip}>
                        <Icon iconType={FaFileAlt} />
                    </ModActionButton>
                </ModHeader>
                <small className="mod-description">{desc}</small>
            </div>
        );
    }
});

export default RemoteModRow;
