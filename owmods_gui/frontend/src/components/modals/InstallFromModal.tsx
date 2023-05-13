import { commands } from "@commands";
import { OpenFileInput } from "@components/common/FileInput";
import Icon from "@components/common/Icon";
import { useGetTranslation } from "@hooks";
import { listen } from "@tauri-apps/api/event";
import { forwardRef, useEffect, useImperativeHandle, useRef, useState } from "react";
import { BsExclamationTriangleFill } from "react-icons/bs";
import Modal, { ModalHandle } from "./Modal";
import { ProtocolInstallType, ProtocolPayload } from "@types";
import { getCurrent } from "@tauri-apps/api/window";

type SourceType = "UNIQUE_NAME" | "URL" | "ZIP" | "JSON";

const getSourceTypeFromProtocol = (installType: ProtocolInstallType): SourceType | null => {
    switch (installType) {
        case "installMod":
            return "UNIQUE_NAME";
        case "installURL":
            return "URL";
        case "installPreRelease":
            return "UNIQUE_NAME";
        default:
            return null;
    }
};

const InstallFromModal = forwardRef(function InstallFromModal(_: object, ref) {
    const modalRef = useRef<ModalHandle>();
    const [source, setSource] = useState<SourceType>("UNIQUE_NAME");
    const [target, setTarget] = useState<string>("");
    const [prerelease, setPrerelease] = useState<boolean>(false);
    const getTranslation = useGetTranslation();

    useImperativeHandle(
        ref,
        () => ({
            open: () => modalRef.current?.open(),
            close: () => modalRef.current?.close()
        }),
        []
    );

    const lblMap: Record<SourceType, string> = {
        UNIQUE_NAME: getTranslation("UNIQUE_NAME"),
        URL: getTranslation("URL"),
        ZIP: getTranslation("ZIP"),
        JSON: getTranslation("JSON")
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
            case "JSON":
                commands
                    .importMods({ path: target })
                    .then(() => commands.refreshLocalDb())
                    .catch(console.error);
        }
    };

    useEffect(() => {
        let cancel = false;
        listen("PROTOCOL_INVOKE", ({ payload }) => {
            if (cancel) return;
            const protocolPayload = payload as ProtocolPayload;
            const sourceType = getSourceTypeFromProtocol(protocolPayload.installType);
            if (sourceType !== null) {
                setSource(sourceType);
                setTarget(protocolPayload.payload);
                if (
                    protocolPayload.installType === "installPreRelease" ||
                    protocolPayload.installType === "installMod"
                ) {
                    setPrerelease(protocolPayload.installType === "installPreRelease");
                }
                modalRef.current?.open();
                getCurrent().setFocus().catch(console.warn);
            }
        })
            .then(() => commands.popProtocolURL())
            .catch(console.warn);
        return () => {
            cancel = true;
        };
    }, []);

    return (
        <Modal
            onConfirm={onInstall}
            showCancel
            ref={modalRef}
            heading={getTranslation("INSTALL_FROM")}
            confirmText={getTranslation("INSTALL")}
        >
            <form>
                <label htmlFor="source">
                    {getTranslation("INSTALL_FROM")}
                    <select
                        onChange={(e) => {
                            setTarget("");
                            setSource(e.target.value as SourceType);
                        }}
                        value={source}
                        id="source"
                    >
                        <option value="UNIQUE_NAME">{getTranslation("UNIQUE_NAME")}</option>
                        <option value="JSON">{getTranslation("JSON")}</option>
                        <option value="URL">{getTranslation("URL")}</option>
                        <option value="ZIP">{getTranslation("ZIP")}</option>
                    </select>
                </label>
                {source === "ZIP" || source === "JSON" ? (
                    <OpenFileInput
                        id={source}
                        value={target}
                        onChange={setTarget}
                        dialogOptions={{
                            title: getTranslation("INSTALL_FROM"),
                            filters: [
                                {
                                    name: lblMap[source],
                                    extensions: [source === "ZIP" ? "zip" : "json"]
                                }
                            ],
                            directory: false,
                            multiple: false
                        }}
                    />
                ) : (
                    <label htmlFor="target">
                        {lblMap[source]}
                        <input
                            id="target"
                            name="target"
                            value={target}
                            onChange={(e) => setTarget(e.target.value)}
                        />
                    </label>
                )}
                {source === "UNIQUE_NAME" && (
                    <label htmlFor="prerelease">
                        <input
                            id="prerelease"
                            onChange={(e) => setPrerelease(e.target.checked)}
                            checked={prerelease}
                            type="checkbox"
                            role="switch"
                        />
                        {getTranslation("USE_PRERELEASE", { version: "" })}
                    </label>
                )}
                {(source === "ZIP" || source === "URL") && (
                    <p className="install-warning">
                        <Icon iconType={BsExclamationTriangleFill} />
                        {getTranslation("INSTALL_WARNING")}
                    </p>
                )}
            </form>
        </Modal>
    );
});

export default InstallFromModal;
