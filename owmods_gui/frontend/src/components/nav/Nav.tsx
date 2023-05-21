import {
    BsPlayFill,
    BsGearFill,
    BsInfoCircleFill,
    BsBoxArrowInDown,
    BsBoxArrowUpRight,
    BsQuestionLg
} from "react-icons/bs";

import NavButton from "@components/nav/NavButton";
import { IconContext } from "react-icons";
import Icon from "@components/common/Icon";
import NavMore from "./NavMore";
import { useCallback, useRef, useState } from "react";
import SettingsModal from "@components/modals/SettingsModal";
import InstallFromModal from "@components/modals/InstallFromModal";
import AboutModal from "@components/modals/AboutModal";
import Downloads from "../downloads/Downloads";
import { useGetTranslation } from "@hooks";
import { commands } from "@commands";
import { dialog, shell } from "@tauri-apps/api";
import CenteredSpinner from "@components/common/CenteredSpinner";
import NavRefreshButton from "./NavRefresh";
import { ModalHandle } from "@components/modals/Modal";

const Nav = () => {
    const settingsRef = useRef<ModalHandle>();
    const installFromRef = useRef<ModalHandle>();
    const aboutRef = useRef<ModalHandle>();
    const getTranslation = useGetTranslation();

    const [areLogsStarting, setLogsStarting] = useState<boolean>(false);

    const onPlay = useCallback(() => {
        const start = () =>
            commands
                .startLogs()
                .then(() => setLogsStarting(false))
                .catch(console.warn);
        setLogsStarting(true);
        const task = async () => {
            const hasIssues = await commands.checkDBForIssues();
            const skipWarning = (await commands.getGuiConfig()).noWarning;
            if (!skipWarning && hasIssues) {
                dialog
                    .ask(getTranslation("LAUNCH_ANYWAY"), {
                        type: "warning",
                        title: getTranslation("CONFIRM")
                    })
                    .then((yes) => {
                        if (yes) {
                            start();
                        } else {
                            setLogsStarting(false);
                        }
                    });
            } else {
                start();
            }
        };
        task();
    }, [getTranslation]);

    const onExport = useCallback(() => {
        dialog
            .save({
                title: getTranslation("EXPORT_MODS"),
                filters: [
                    {
                        name: "JSON File",
                        extensions: ["json"]
                    }
                ]
            })
            .then((path) => {
                if (path) {
                    commands.exportMods({ path }).catch(console.error);
                }
            });
    }, [getTranslation]);

    const onHelp = useCallback(() => {
        shell.open("https://github.com/Bwc9876/ow-mod-man/blob/main/owmods_gui/HELP.md");
    }, []);

    return (
        <IconContext.Provider value={{ className: "nav-icon" }}>
            <SettingsModal ref={settingsRef} />
            <InstallFromModal ref={installFromRef} />
            <AboutModal ref={aboutRef} />
            <div className="nav-wrapper">
                <nav className="max-width">
                    <ul>
                        <Downloads />
                        <NavRefreshButton />
                    </ul>
                    <ul>
                        {areLogsStarting ? (
                            <CenteredSpinner />
                        ) : (
                            <NavButton
                                onClick={onPlay}
                                labelPlacement="bottom"
                                ariaLabel={getTranslation("RUN_GAME")}
                            >
                                <Icon iconClassName="main-icon" iconType={BsPlayFill} />
                            </NavButton>
                        )}
                    </ul>
                    <ul>
                        <NavButton
                            onClick={onHelp}
                            labelPlacement="bottom"
                            ariaLabel={getTranslation("HELP")}
                        >
                            <Icon iconType={BsQuestionLg} />
                        </NavButton>
                        <NavMore>
                            {/* Dropdown uses RTL */}
                            <NavButton onClick={() => settingsRef.current?.open()}>
                                {getTranslation("SETTINGS")} <Icon iconType={BsGearFill} />
                            </NavButton>
                            <NavButton onClick={() => installFromRef.current?.open()}>
                                ...{getTranslation("INSTALL_FROM")}{" "}
                                <Icon iconType={BsBoxArrowInDown} />
                            </NavButton>
                            <NavButton onClick={onExport}>
                                {getTranslation("EXPORT_MODS")}{" "}
                                <Icon iconType={BsBoxArrowUpRight} />
                            </NavButton>
                            <NavButton onClick={() => aboutRef.current?.open()}>
                                {getTranslation("ABOUT")} <Icon iconType={BsInfoCircleFill} />
                            </NavButton>
                        </NavMore>
                    </ul>
                </nav>
            </div>
        </IconContext.Provider>
    );
};

export default Nav;
