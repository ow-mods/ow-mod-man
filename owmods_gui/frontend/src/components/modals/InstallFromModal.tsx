import { useTranslations } from "@hooks";
import { useState } from "react";
import Modal, { ModalWrapperProps } from "./Modal";

type SourceType = "URL" | "ZIP";

const InstallFromModal = (props: ModalWrapperProps) => {
    const [source, setSource] = useState<SourceType>("URL");
    const [target, setTarget] = useState<string>("");

    const [install, installFrom, zip, url] = useTranslations([
        "INSTALL",
        "INSTALL_FROM",
        "ZIP",
        "URL"
    ]);

    return (
        <Modal showCancel open={props.open} heading={installFrom} confirmText={install}>
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
