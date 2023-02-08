import Icon from "@components/Icon";
import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { useTauri } from "@hooks";
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

    if (status === "Loading") {
        return <div className="mod-row center-loading" aria-busy style={props.style}></div>;
    } else if (status === "Error") {
        return <p style={props.style}>{err}</p>;
    } else {
        const remote_mod = mod!;
        let desc = remote_mod.description ?? "No Description Provided";
        if (desc.trim() === "") desc = "No Description Provided";
        return (
            <div style={props.style} className="mod-row">
                <ModHeader {...remote_mod} author={remote_mod.authorDisplay ?? remote_mod.author}>
                    <small>{formatNumber(remote_mod.downloadCount)}</small>
                    <ModActionButton ariaLabel="Install With Dependencies">
                        <Icon iconType={FaArrowDown} />
                    </ModActionButton>
                    <ModActionButton ariaLabel="View On Website">
                        <Icon iconType={FaGlobe} />
                    </ModActionButton>
                </ModHeader>
                <small className="mod-description">{desc}</small>
            </div>
        );
    }
});

export default RemoteModRow;
