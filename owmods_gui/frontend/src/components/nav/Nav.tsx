import {
    BsPlayFill,
    BsQuestion,
    BsGearFill,
    BsInfoCircleFill,
    BsBoxArrowInDown,
    BsBoxArrowUpRight
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
import { useTranslations } from "@hooks";
import { commands } from "@commands";
import { dialog } from "@tauri-apps/api";
import CenteredSpinner from "@components/common/CenteredSpinner";
import NavRefreshButton from "./NavRefresh";
import { ModalHandle } from "@components/modals/Modal";

const Nav = () => {
    const settingsRef = useRef<ModalHandle>();
    const installFromRef = useRef<ModalHandle>();
    const aboutRef = useRef<ModalHandle>();

    const [areLogsStarting, setLogsStarting] = useState<boolean>(false);

    const [runGame, help, settings, installFrom, about, exportLabel, confirm, launchAnyway] =
        useTranslations([
            "RUN_GAME",
            "HELP",
            "SETTINGS",
            "INSTALL_FROM",
            "ABOUT",
            "EXPORT_MODS",
            "CONFIRM",
            "LAUNCH_ANYWAY"
        ]);

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
                    .ask(launchAnyway, {
                        type: "warning",
                        title: confirm
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
    }, [confirm, launchAnyway]);

    const onExport = useCallback(() => {
        dialog
            .save({
                title: exportLabel,
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
    }, [exportLabel]);

    return (
        <IconContext.Provider value={{ className: "nav-icon" }}>
            <SettingsModal ref={settingsRef} />
            <InstallFromModal ref={installFromRef} />
            <AboutModal ref={aboutRef} />
            <nav>
                <ul>
                    <Downloads />
                    <NavRefreshButton />
                </ul>
                <ul>
                    {areLogsStarting ? (
                        <CenteredSpinner />
                    ) : (
                        <NavButton onClick={onPlay} labelPlacement="bottom" ariaLabel={runGame}>
                            <Icon iconClassName="main-icon" iconType={BsPlayFill} />
                        </NavButton>
                    )}
                </ul>
                <ul>
                    <NavButton labelPlacement="bottom" ariaLabel={help}>
                        <Icon iconType={BsQuestion} />
                    </NavButton>
                    <NavMore>
                        {/* Dropdown uses RTL */}
                        <NavButton onClick={() => settingsRef.current?.open()}>
                            {settings} <Icon iconType={BsGearFill} />
                        </NavButton>
                        <NavButton onClick={() => installFromRef.current?.open()}>
                            ...{installFrom} <Icon iconType={BsBoxArrowInDown} />
                        </NavButton>
                        <NavButton onClick={onExport}>
                            {exportLabel} <Icon iconType={BsBoxArrowUpRight} />
                        </NavButton>
                        <NavButton onClick={() => aboutRef.current?.open()}>
                            {about} <Icon iconType={BsInfoCircleFill} />
                        </NavButton>
                    </NavMore>
                </ul>
            </nav>
        </IconContext.Provider>
    );
};

export default Nav;
