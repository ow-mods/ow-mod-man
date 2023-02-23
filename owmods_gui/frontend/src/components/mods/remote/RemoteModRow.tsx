import { commands, hooks } from "@commands";
import Icon from "@components/Icon";
import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { useTranslation, useTranslations } from "@hooks";
import { CSSProperties, memo, useCallback, useState } from "react";
import { FaArrowDown, FaFileAlt } from "react-icons/fa";

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
    const [status, mod, err] = hooks.getRemoteMod("REMOTE-REFRESH", {
        uniqueName: props.uniqueName
    });

    const [downloading, setDownloading] = useState(false);

    const [noDescription, installTooltip, websiteTooltip] = useTranslations([
        "NO_DESCRIPTION",
        "INSTALL",
        "OPEN_README"
    ]);

    const subtitle = useTranslation("BY", { author: mod?.authorDisplay ?? mod?.author ?? "" });

    const onInstall = useCallback(() => {
        setDownloading(true);
        commands
            .installMod({ uniqueName: props.uniqueName })
            .then(() => {
                setDownloading(false);
                commands.refreshLocalDb().catch(console.error);
            })
            .catch(console.error);
    }, [props.uniqueName]);

    const onReadme = useCallback(() => {
        commands.openModReadme({ uniqueName: props.uniqueName }).catch(console.warn);
    }, [props.uniqueName]);

    if (status === "Loading" && mod === null) {
        return <div className="mod-row center-loading" aria-busy style={props.style}></div>;
    } else if (status === "Error") {
        return (
            <p className="mod-row center-loading" style={props.style}>
                {err!.toString()}
            </p>
        );
    } else {
        const remoteMod = mod!;
        let desc = remoteMod.description ?? noDescription;
        if (desc.trim() === "") desc = noDescription;
        return (
            <div style={props.style} className="mod-row">
                <ModHeader {...remoteMod} subtitle={subtitle}>
                    <small>{formatNumber(remoteMod.downloadCount)}</small>
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
