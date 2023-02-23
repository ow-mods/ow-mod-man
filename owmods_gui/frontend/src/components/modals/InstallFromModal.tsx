import { commands } from "@commands";
import Icon from "@components/Icon";
import { useTranslations } from "@hooks";
import { dialog } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { FaExclamationTriangle, FaFileArchive } from "react-icons/fa";
import Modal, { ModalWrapperProps } from "./Modal";

type SourceType = "UNIQUE_NAME" | "URL" | "ZIP";

const InstallFromModal = (props: ModalWrapperProps) => {
    const [source, setSource] = useState<SourceType>("UNIQUE_NAME");
    const [target, setTarget] = useState<string>("");
    const [prerelease, setPrerelease] = useState<boolean>(false);

    const [install, installFrom, uniqueNameLabel, zip, url, warningText, browse] = useTranslations([
        "INSTALL",
        "INSTALL_FROM",
        "UNIQUE_NAME",
        "ZIP",
        "URL",
        "INSTALL_WARNING",
        "BROWSE"
    ]);

    const lblMap: Record<SourceType, string> = {
        UNIQUE_NAME: uniqueNameLabel,
        URL: url,
        ZIP: zip
    };

    const onInstall = () => {
        switch (source) {
            case "UNIQUE_NAME":
                commands
                    .installMod({ uniqueName: target, prerelease })
                    .then(() => commands.refreshLocalDb())
                    .catch(console.error);
                break;
            case "URL":
                commands
                    .installUrl({ url: target })
                    .then(() => commands.refreshLocalDb())
                    .catch(console.error);
                break;
            case "ZIP":
                commands
                    .installZip({ path: target })
                    .then(() => commands.refreshLocalDb())
                    .catch(console.error);
                break;
        }
    };

    const onBrowse = () => {
        dialog
            .open({
                title: installFrom,
                filters: [
                    {
                        name: zip,
                        extensions: ["zip"]
                    }
                ],
                directory: false,
                multiple: false
            })
            .then((path) => {
                if (path !== null) {
                    setTarget(path as string);
                }
            });
    };

    useEffect(() => {
        let cancel = false;
        listen("PROTOCOL_INSTALL_URL", ({ payload }) => {
            if (cancel) return;
            setSource("URL");
            setTarget(payload as string);
            props.open?.current();
        }).catch(console.warn);
        listen("PROTOCOL_INSTALL_UNIQUE_NAME", ({ payload }) => {
            if (cancel) return;
            setSource("UNIQUE_NAME");
            setTarget(payload as string);
            props.open?.current();
        });
        return () => {
            cancel = true;
        };
    }, []);

    return (
        <Modal
            onConfirm={onInstall}
            showCancel
            open={props.open}
            heading={installFrom}
            confirmText={install}
        >
            <form>
                <label htmlFor="source">
                    {installFrom}
                    <select
                        onChange={(e) => {
                            setTarget("");
                            setSource(e.target.value as SourceType);
                        }}
                        id="source"
                    >
                        <option value="UNIQUE_NAME">{uniqueNameLabel}</option>
                        <option value="URL">{url}</option>
                        <option value="ZIP">{zip}</option>
                    </select>
                </label>
                <label htmlFor="target">
                    {lblMap[source]}
                    <div className={source === "ZIP" ? "install-source" : ""}>
                        <input
                            id="target"
                            name="target"
                            value={target}
                            onChange={(e) => setTarget(e.target.value)}
                        />
                        {source === "ZIP" && (
                            <button onClick={onBrowse} className="fix-icons" type="button">
                                <Icon iconType={FaFileArchive} /> {browse}
                            </button>
                        )}
                    </div>
                </label>
                {source === "UNIQUE_NAME" && (
                    <label htmlFor="prerelease">
                        <input
                            id="prerelease"
                            onChange={(e) => setPrerelease(e.target.checked)}
                            checked={prerelease}
                            type="checkbox"
                            role="switch"
                        />
                        Use Prerelease
                    </label>
                )}
                <p className="install-warning">
                    <Icon iconType={FaExclamationTriangle} />
                    {warningText}
                </p>
            </form>
        </Modal>
    );
};

export default InstallFromModal;
