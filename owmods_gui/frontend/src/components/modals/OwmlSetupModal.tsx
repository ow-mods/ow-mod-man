import { commands } from "@commands";
import { OpenFileInput } from "@components/common/FileInput";
import { useTranslations } from "@hooks";
import { dialog } from "@tauri-apps/api";
import { forwardRef, useRef, useState } from "react";
import Modal, { ModalHandle } from "./Modal";

type SetupMethod = "Install" | "Locate";

const OwmlSetupModal = forwardRef(function OwmlSetupModal(_: object, ref) {
    const modalRef = useRef<ModalHandle>();
    const [setupMethod, setSetupMethod] = useState<SetupMethod>("Install");
    const [owmlPath, setOwmlPath] = useState("");

    const [setup, message, installOwml, locateOwml, invalidOwml, continueLabel] = useTranslations([
        "SETUP",
        "OWML_SETUP_MESSAGE",
        "INSTALL_OWML",
        "LOCATE_OWML",
        "INVALID_OWML",
        "CONTINUE"
    ]);

    const onClose = () => {
        if (setupMethod === "Install") {
            commands
                .installOwml()
                .then(() => {
                    modalRef.current?.close();
                    window.location.reload();
                })
                .catch(dialog.message);
        } else {
            commands
                .setOwml({ path: owmlPath })
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
            ref={ref}
            onConfirm={onClose}
            heading={setup}
            showCancel={false}
            confirmText={continueLabel}
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
                    <OpenFileInput
                        id="OWML_PATH"
                        value={owmlPath}
                        onChange={setOwmlPath}
                        dialogOptions={{
                            directory: true,
                            multiple: false,
                            title: locateOwml
                        }}
                    />
                )}
            </form>
        </Modal>
    );
});

export default OwmlSetupModal;
