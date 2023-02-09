import Icon from "@components/Icon";
import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { useTauri, useTranslations } from "@hooks";
import { CSSProperties, memo } from "react";
import { FaArrowDown, FaGlobe } from "react-icons/fa";
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

const RemoteModRow = memo((props: { uniqueName: string; style?: CSSProperties }) => {
    const [status, mod, err] = useTauri<RemoteMod, { uniqueName: string }>(
        "REMOTE-REFRESH",
        "get_remote_mod",
        { uniqueName: props.uniqueName }
    );

    const [noDescription, installTooltip, websiteTooltip] = useTranslations([
        "NO_DESCRIPTION",
        "INSTALL",
        "OPEN_WEBSITE"
    ]);

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
                    <ModActionButton ariaLabel={installTooltip}>
                        <Icon iconType={FaArrowDown} />
                    </ModActionButton>
                    <ModActionButton ariaLabel={websiteTooltip}>
                        <Icon iconType={FaGlobe} />
                    </ModActionButton>
                </ModHeader>
                <small className="mod-description">{desc}</small>
            </div>
        );
    }
});

export default RemoteModRow;
