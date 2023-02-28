import Modal, { ModalWrapperProps } from "./Modal";
import logo from "@assets/images/logo.png";
import Icon from "@components/common/Icon";
import { BsDiscord, BsGithub } from "react-icons/bs";
import { useTranslation, useTranslations } from "@hooks";
import { app, os, shell } from "@tauri-apps/api";
import { useEffect, useState } from "react";

const AboutModal = (props: ModalWrapperProps) => {
    const [heading, dismiss, appName, gitHub, discord] = useTranslations([
        "ABOUT",
        "DISMISS",
        "APP_TITLE",
        "GITHUB",
        "DISCORD"
    ]);

    const [appVersion, setVersion] = useState("");
    const [appPlatform, setPlatform] = useState("");

    const version = useTranslation("VERSION", { version: appVersion });
    const platform = useTranslation("PLATFORM", { platform: appPlatform });

    useEffect(() => {
        app.getVersion().then(setVersion);
        os.platform().then(setPlatform);
    }, []);

    return (
        <Modal open={props.open} heading={heading} confirmText={dismiss}>
            <div className="about-modal">
                <img width="256" height="256" src={logo} />
                <h1>{appName}</h1>
                <p>{version}</p>
                <p>{platform}</p>
                <div className="links">
                    <a
                        onClick={() => shell.open("https://github.com/Bwc9876/ow-mod-man/")}
                        href="#"
                        role="button"
                        className="fix-icons"
                    >
                        <Icon iconType={BsGithub} />
                        {gitHub}
                    </a>
                    <a
                        onClick={() => shell.open("https://discord.gg/outerwildsmodding")}
                        href="#"
                        role="button"
                        className="fix-icons"
                    >
                        <Icon iconType={BsDiscord} />
                        {discord}
                    </a>
                </div>
            </div>
        </Modal>
    );
};

export default AboutModal;
