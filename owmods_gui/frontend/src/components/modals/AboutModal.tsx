import Modal from "./Modal";
import logo from "@assets/images/logo.png";
import Icon from "@components/common/Icon";
import { BsDiscord, BsGithub } from "react-icons/bs";
import { useGetTranslation } from "@hooks";
import { app, os, shell } from "@tauri-apps/api";
import { forwardRef, useEffect, useState } from "react";

const AboutModal = forwardRef(function AboutModal(_: object, ref) {
    const [appVersion, setVersion] = useState("");
    const [appPlatform, setPlatform] = useState("");
    const [archRaw, setArch] = useState("");
    const getTranslation = useGetTranslation();

    useEffect(() => {
        app.getVersion().then(setVersion);
        os.platform().then(setPlatform);
        os.arch().then(setArch);
    }, []);

    return (
        <Modal ref={ref} heading={getTranslation("ABOUT")} confirmText={getTranslation("DISMISS")}>
            <div className="about-modal">
                <img width="256" height="256" src={logo} />
                <h1>{getTranslation("APP_TITLE")}</h1>
                <p>{getTranslation("VERSION", { version: appVersion })}</p>
                <p>{getTranslation("PLATFORM", { platform: appPlatform })}</p>
                <p>{getTranslation("ARCHITECTURE", { arch: archRaw })}</p>
                <div className="links">
                    <a
                        onClick={() => shell.open("https://github.com/Bwc9876/ow-mod-man/")}
                        href="#"
                        role="button"
                        className="fix-icons"
                    >
                        <Icon iconType={BsGithub} />
                        {getTranslation("GITHUB")}
                    </a>
                    <a
                        onClick={() => shell.open("https://discord.gg/outerwildsmodding")}
                        href="#"
                        role="button"
                        className="fix-icons"
                    >
                        <Icon iconType={BsDiscord} />
                        {getTranslation("DISCORD")}
                    </a>
                </div>
            </div>
        </Modal>
    );
});

export default AboutModal;
