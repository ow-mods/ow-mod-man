import { useState } from "react";
import Modal, { ModalWrapperProps } from "./Modal";

type SourceType = "URL" | "Zip File";

const InstallFromModal = (props: ModalWrapperProps) => {
    const [source, setSource] = useState<SourceType>("URL");
    const [target, setTarget] = useState<string>("");

    return (
        <Modal showCancel open={props.open} heading="Install Mod From" confirmText="Install">
            <form>
                <label htmlFor="source">
                    Install From
                    <select
                        onChange={(e) => {
                            setTarget("");
                            setSource(e.target.value as SourceType);
                        }}
                        id="source"
                    >
                        <option>URL</option>
                        <option>Zip File</option>
                    </select>
                </label>
                <label htmlFor="target">
                    {source}
                    <input
                        id="target"
                        name="target"
                        type={source === "URL" ? "string" : "file"}
                        value={target}
                        onChange={(e) => setTarget(e.target.value)}
                    />
                </label>
            </form>
        </Modal>
    );
};

export default InstallFromModal;
