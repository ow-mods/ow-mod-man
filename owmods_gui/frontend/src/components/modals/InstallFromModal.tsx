import { commands } from "@commands";
import { OpenFileInput } from "@components/common/FileInput";
import Icon from "@components/common/Icon";
import { useTranslations } from "@hooks";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { BsExclamationTriangleFill } from "react-icons/bs";
import Modal, { ModalWrapperProps } from "./Modal";
import { ProtocolInstallType, ProtocolPayload } from "@types";
import { UserAttentionType, getCurrent } from "@tauri-apps/api/window";

type SourceType = "UNIQUE_NAME" | "URL" | "ZIP" | "JSON";

const getSourceTypeFromProtocol = (installType: ProtocolInstallType): SourceType | null => {
    console.debug(installType);
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

const InstallFromModal = (props: ModalWrapperProps) => {
    const [source, setSource] = useState<SourceType>("UNIQUE_NAME");
    const [target, setTarget] = useState<string>("");
    const [prerelease, setPrerelease] = useState<boolean>(false);

    const [install, installFrom, uniqueNameLabel, jsonLabel, zip, url, warningText, usePrerelease] =
        useTranslations([
            "INSTALL",
            "INSTALL_FROM",
            "UNIQUE_NAME",
            "JSON",
            "ZIP",
            "URL",
            "INSTALL_WARNING",
            "USE_PRERELEASE"
        ]);

    const lblMap: Record<SourceType, string> = {
        UNIQUE_NAME: uniqueNameLabel,
        URL: url,
        ZIP: zip,
        JSON: jsonLabel
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
                props.open?.current();
                getCurrent().requestUserAttention(UserAttentionType.Informational);
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
                        value={source}
                        id="source"
                    >
                        <option value="UNIQUE_NAME">{uniqueNameLabel}</option>
                        <option value="JSON">{jsonLabel}</option>
                        <option value="URL">{url}</option>
                        <option value="ZIP">{zip}</option>
                    </select>
                </label>
                {source === "ZIP" || source === "JSON" ? (
                    <OpenFileInput
                        id={source}
                        value={target}
                        onChange={setTarget}
                        dialogOptions={{
                            title: installFrom,
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
                        {usePrerelease}
                    </label>
                )}
                {(source === "ZIP" || source === "URL") && (
                    <p className="install-warning">
                        <Icon iconType={BsExclamationTriangleFill} />
                        {warningText}
                    </p>
                )}
            </form>
        </Modal>
    );
};

export default InstallFromModal;
