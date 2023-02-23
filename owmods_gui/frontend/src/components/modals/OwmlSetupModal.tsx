import { commands } from "@commands";
import Icon from "@components/Icon";
import { useTranslations } from "@hooks";
import { dialog } from "@tauri-apps/api";
import { useRef, useState } from "react";
import { FaFolder } from "react-icons/fa";
import Modal, { ModalWrapperProps } from "./Modal";

type SetupMethod = "Install" | "Locate";

const OwmlSetupModal = (props: ModalWrapperProps) => {
    const [setupMethod, setSetupMethod] = useState<SetupMethod>("Install");
    const [owmlPath, setOwmlPath] = useState("");
    const closeModal = useRef<() => void>(() => null);

    const [setup, message, installOwml, locateOwml, browse, owmlPathLabel, invalidOwml] =
        useTranslations([
            "SETUP",
            "OWML_SETUP_MESSAGE",
            "INSTALL_OWML",
            "LOCATE_OWML",
            "BROWSE",
            "OWML_PATH",
            "INVALID_OWML"
        ]);

    const onBrowse = () => {
        dialog
            .open({
                directory: true,
                multiple: false,
                title: locateOwml
            })
            .then((path) => {
                if (path) {
                    setOwmlPath(path as string);
                }
            });
    };

    const onClose = () => {
        if (setupMethod === "Install") {
            commands
                .install_owml()
                .then(() => {
                    closeModal.current();
                    window.location.reload();
                })
                .catch(dialog.message);
        } else {
            commands
                .set_owml({ path: owmlPath })
                .then((valid) => {
                    if (valid) {
                        window.location.reload();
                    } else {
                        dialog.message(invalidOwml).then(() => window.location.reload());
                    }
                })
                .catch(dialog.message);
        }
        return false;
    };

    return (
        <Modal
            open={props.open}
            close={closeModal}
            onConfirm={onClose}
            heading={setup}
            showCancel={false}
            confirmText="Continue"
        >
            <form className="owml-setup">
                <p>{message}</p>
                <select
                    value={setupMethod}
                    onChange={(e) => setSetupMethod(e.target.value as SetupMethod)}
                >
                    <option value="Install">{installOwml}</option>
                    <option value="Locate">{locateOwml}</option>
                </select>
                {setupMethod === "Locate" && (
                    <>
                        <label htmlFor="owmlPath">{owmlPathLabel}</label>
                        <div>
                            <input
                                id="owmlPath"
                                value={owmlPath}
                                onChange={(e) => setOwmlPath(e.target.value)}
                                className="settings-folder"
                            ></input>
                            <button onClick={onBrowse} type="button" className="fix-icons">
                                <Icon iconType={FaFolder} /> {browse}
                            </button>
                        </div>
                    </>
                )}
            </form>
        </Modal>
    );
};

export default OwmlSetupModal;
