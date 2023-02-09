import Modal, { ModalWrapperProps } from "./Modal";
import logo from "@assets/images/logo.png";
import Icon from "@components/Icon";
import { FaGithub } from "react-icons/fa";
import { useTranslations } from "@hooks";

// TODO: When integrating with tauri, replace #var# with calls to the backend
// TODO: Also the github button will change the webview to the url, uh don't do that
const AboutModal = (props: ModalWrapperProps) => {
    const [heading, dismiss, appName, version, platform] = useTranslations([
        "ABOUT",
        "DISMISS",
        "APP_TITLE",
        "VERSION",
        "PLATFORM"
    ]);

    return (
        <Modal open={props.open} heading={heading} confirmText={dismiss}>
            <div className="about-modal">
                <img width="256" height="256" src={logo} />
                <h1>{appName}</h1>
                <p>{version}: #VERSION#</p>
                <p>{platform}: #PLATFORM#</p>
                <a href="https://github.com/Bwc9876/ow-mod-man" role="button" className="fix-icons">
                    <Icon iconType={FaGithub} />
                    GitHub
                </a>
            </div>
        </Modal>
    );
};

export default AboutModal;
