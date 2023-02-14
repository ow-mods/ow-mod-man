import Icon from "@components/Icon";
import { useTranslations } from "@hooks";
import { useState } from "react";
import { FaFolder } from "react-icons/fa";
import Modal, { ModalWrapperProps } from "./Modal";

type SetupMethod = "Install" | "Locate";

const OwmlSetupModal = (props: ModalWrapperProps) => {
    const [setupMethod, setSetupMethod] = useState<SetupMethod>("Install");
    const [owmlPath, setOwmlPath] = useState("");

    const [setup, message, installOwml, locateOwml, browse, owmlPathLabel] = useTranslations(["SETUP", "OWML_SETUP_MESSAGE", "INSTALL_OWML", "LOCATE_OWML", "BROWSE", "OWML_PATH"]);

        return <Modal open={props.open} heading={setup} showCancel={false} confirmText="Continue">
            <form className="owml-setup">
                <p>{message}</p>
                <select value={setupMethod} onChange={(e) => setSetupMethod(e.target.value as SetupMethod)}>
                    <option value="Install">{installOwml}</option>
                    <option value="Locate">{locateOwml}</option>
                </select>
                {setupMethod === "Locate" && ( 
                    <>
                    <label htmlFor="owmlPath">{owmlPathLabel}</label>
                    <div>
                        <input id="owmlPath" value={owmlPath} onChange={e => setOwmlPath(e.target.value)} className="settings-folder"></input>
                        <button className="fix-icons">
                            <Icon iconType={FaFolder} /> {browse} 
                        </button>
                    </div>
                    </>
                )}
            </form>
        </Modal>
};

export default OwmlSetupModal;
