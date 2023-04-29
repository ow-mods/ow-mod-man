import { commands, hooks } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import Icon from "@components/common/Icon";
import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { useGetTranslation } from "@hooks";
import { dialog } from "@tauri-apps/api";
import { memo, useCallback, useState } from "react";
import { BsArrowDown, BsGlobe, BsHammer } from "react-icons/bs";

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
}

const RemoteModRow = memo(function RemoteModRow(props: RemoteModRowProps) {
    const getTranslation = useGetTranslation();
    const [status, mod, err] = hooks.getRemoteMod("REMOTE-REFRESH", {
        uniqueName: props.uniqueName
    });

    const [downloading, setDownloading] = useState(false);

    const usePrerelease = getTranslation("USE_PRERELEASE", {
        version: mod?.prerelease?.version ?? ""
    });

    const subtitle = getTranslation("BY", {
        author: mod?.authorDisplay ?? mod?.author ?? "",
        version: mod?.version ?? ""
    });

    const onInstall = useCallback(() => {
        setDownloading(true);
        commands
            .installMod({ uniqueName: props.uniqueName })
            .then(() => {
                commands.refreshLocalDb().catch(console.error);
            })
            .catch(console.error)
            .finally(() => setDownloading(false));
    }, [props.uniqueName]);

    const onPrerelease = useCallback(() => {
        const task = async () => {
            const result = await dialog.ask(getTranslation("PRERELEASE_WARNING"), {
                title: usePrerelease
            });
            if (result) {
                setDownloading(true);
                commands
                    .installMod({ uniqueName: props.uniqueName, prerelease: true })
                    .then(() => {
                        setDownloading(false);
                        commands.refreshLocalDb().catch(console.error);
                    })
                    .catch(console.error);
            }
        };
        task();
    }, [getTranslation, usePrerelease, props.uniqueName]);

    const onReadme = useCallback(() => {
        commands.openModReadme({ uniqueName: props.uniqueName }).catch(console.warn);
    }, [props.uniqueName]);

    if (status === "Loading" && mod === null) {
        return <CenteredSpinner className="loading" />;
    } else if (status === "Error") {
        return <p className="center">{err!.toString()}</p>;
    } else {
        const remoteMod = mod!;
        let desc = remoteMod.description ?? getTranslation("NO_DESCRIPTION");
        if (desc.trim() === "") desc = getTranslation("NO_DESCRIPTION");
        return (
            <div>
                <ModHeader {...remoteMod} subtitle={subtitle}>
                    <small>{formatNumber(remoteMod.downloadCount)}</small>
                    {downloading ? (
                        <div className="center" aria-busy></div>
                    ) : (
                        <>
                            <ModActionButton
                                onClick={onInstall}
                                ariaLabel={getTranslation("INSTALL")}
                            >
                                <Icon iconType={BsArrowDown} />
                            </ModActionButton>
                            {mod?.prerelease !== null && (
                                <ModActionButton onClick={onPrerelease} ariaLabel={usePrerelease}>
                                    <Icon iconType={BsHammer} />
                                </ModActionButton>
                            )}
                        </>
                    )}
                    <ModActionButton onClick={onReadme} ariaLabel={getTranslation("OPEN_WEBSITE")}>
                        <Icon iconType={BsGlobe} />
                    </ModActionButton>
                </ModHeader>
                <small className="mod-description">{desc}</small>
            </div>
        );
    }
});

const Wrapper = memo(function Wrapper(props: RemoteModRowProps) {
    return (
        <div className="mod-row remote">
            <RemoteModRow {...props} />
        </div>
    );
});

export default Wrapper;
