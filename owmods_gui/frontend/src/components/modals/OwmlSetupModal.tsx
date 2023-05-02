import { commands } from "@commands";
import { OpenFileInput } from "@components/common/FileInput";
import { useGetTranslation } from "@hooks";
import { dialog } from "@tauri-apps/api";
import { forwardRef, useRef, useState } from "react";
import Modal, { ModalHandle } from "./Modal";

type SetupMethod = "Install" | "Locate";

const OwmlSetupModal = forwardRef(function OwmlSetupModal(_: object, ref) {
    const modalRef = useRef<ModalHandle>();
    const [setupMethod, setSetupMethod] = useState<SetupMethod>("Install");
    const [owmlPath, setOwmlPath] = useState("");
    const getTranslation = useGetTranslation();

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
                        dialog
                            .message(getTranslation("INVALID_OWML"))
                            .then(() => window.location.reload());
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
            heading={getTranslation("SETUP")}
            showCancel={false}
            confirmText={getTranslation("CONTINUE")}
        >
            <form className="owml-setup">
                <p>{getTranslation("OWML_SETUP_MESSAGE")}</p>
                <select
                    value={setupMethod}
                    onChange={(e) => setSetupMethod(e.target.value as SetupMethod)}
                >
                    <option value="Install">{getTranslation("INSTALL_OWML")}</option>
                    <option value="Locate">{getTranslation("LOCATE_OWML")}</option>
                </select>
                {setupMethod === "Locate" && (
                    <OpenFileInput
                        id="OWML_PATH"
                        value={owmlPath}
                        onChange={setOwmlPath}
                        dialogOptions={{
                            directory: true,
                            multiple: false,
                            title: getTranslation("LOCATE_OWML")
                        }}
                    />
                )}
            </form>
        </Modal>
    );
});

export default OwmlSetupModal;
