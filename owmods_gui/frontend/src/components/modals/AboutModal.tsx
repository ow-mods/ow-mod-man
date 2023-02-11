import Modal, { ModalWrapperProps } from "./Modal";
import logo from "@assets/images/logo.png";
import Icon from "@components/Icon";
import { FaGithub } from "react-icons/fa";
import { useTranslations } from "@hooks";
import { app, os, shell } from "@tauri-apps/api";
import { useEffect, useState } from "react";

const AboutModal = (props: ModalWrapperProps) => {
    const [heading, dismiss, appName, version, platform] = useTranslations([
        "ABOUT",
        "DISMISS",
        "APP_TITLE",
        "VERSION",
        "PLATFORM"
    ]);

    const [appVersion, setVersion] = useState("");
    const [appPlatform, setPlatform] = useState("");

    useEffect(() => {
        app.getVersion().then(setVersion);
        os.platform().then(setPlatform);
    }, []);

    return (
        <Modal open={props.open} heading={heading} confirmText={dismiss}>
            <div className="about-modal">
                <img width="256" height="256" src={logo} />
                <h1>{appName}</h1>
                <p>
                    {version}: {appVersion}
                </p>
                <p>
                    {platform}: {appPlatform}
                </p>
                <a
                    onClick={() => shell.open("https://github.com/Bwc9876/ow-mod-man/")}
                    href="#"
                    role="button"
                    className="fix-icons"
                >
                    <Icon iconType={FaGithub} />
                    GitHub
                </a>
            </div>
        </Modal>
    );
};

export default AboutModal;
