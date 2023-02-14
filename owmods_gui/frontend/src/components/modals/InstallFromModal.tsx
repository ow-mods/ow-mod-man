import Icon from "@components/Icon";
import { useTranslations } from "@hooks";
import { dialog, invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { FaExclamationTriangle, FaFileArchive } from "react-icons/fa";
import Modal, { ModalWrapperProps } from "./Modal";

type SourceType = "URL" | "ZIP";

const InstallFromModal = (props: ModalWrapperProps) => {
    const [source, setSource] = useState<SourceType>("URL");
    const [target, setTarget] = useState<string>("");

    const [install, installFrom, zip, url, warningText, browse] = useTranslations([
        "INSTALL",
        "INSTALL_FROM",
        "ZIP",
        "URL",
        "INSTALL_WARNING",
        "BROWSE"
    ]);

    const onInstall = () => {
        if (source === "URL") {
            invoke("install_url", { url: target })
                .then(() => invoke("refresh_local_db"))
                .catch(console.error);
        } else {
            invoke("install_zip", { path: target })
                .then(() => invoke("refresh_local_db"))
                .catch(console.error);
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
        listen("PROTOCOL_INSTALL_URL", ({ payload }) => {
            setSource("URL");
            setTarget(payload as string);
            props.open?.current();
        }).catch(console.warn);
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
                        <option value="URL">{url}</option>
                        <option value="ZIP">{zip}</option>
                    </select>
                </label>
                <label htmlFor="target">
                    {source === "URL" ? url : zip}
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
                <p className="install-warning">
                    <Icon iconType={FaExclamationTriangle} />
                    {warningText}
                </p>
            </form>
        </Modal>
    );
};

export default InstallFromModal;
