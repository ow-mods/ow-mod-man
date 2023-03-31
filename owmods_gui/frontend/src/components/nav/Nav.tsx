import {
    BsPlayFill,
    BsQuestion,
    BsGearFill,
    BsInfoCircleFill,
    BsArrowRepeat,
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

const Nav = () => {
    const openSettings = useRef<() => void>(() => null);
    const openInstallFrom = useRef<() => void>(() => null);
    const openAbout = useRef<() => void>(() => null);

    const [areLogsStarting, setLogsStarting] = useState<boolean>(false);

    const [
        refresh,
        runGame,
        help,
        settings,
        installFrom,
        about,
        exportLabel,
        confirm,
        launchAnyway
    ] = useTranslations([
        "REFRESH",
        "RUN_GAME",
        "HELP",
        "SETTINGS",
        "INSTALL_FROM",
        "ABOUT",
        "EXPORT_MODS",
        "CONFIRM",
        "LAUNCH_ANYWAY"
    ]);

    const onRefresh = useCallback(() => {
        commands.refreshLocalDb().catch(console.warn);
        commands.refreshRemoteDb().catch(console.warn);
    }, []);

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
    }, []);

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
            <SettingsModal open={openSettings} />
            <InstallFromModal open={openInstallFrom} />
            <AboutModal open={openAbout} />
            <nav>
                <ul>
                    <Downloads />
                    <NavButton onClick={onRefresh} labelPlacement="bottom" ariaLabel={refresh}>
                        <Icon iconType={BsArrowRepeat} />
                    </NavButton>
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
                        <NavButton onClick={() => openSettings.current?.()}>
                            {settings} <Icon iconType={BsGearFill} />
                        </NavButton>
                        <NavButton onClick={() => openInstallFrom.current?.()}>
                            ...{installFrom} <Icon iconType={BsBoxArrowInDown} />
                        </NavButton>
                        <NavButton onClick={onExport}>
                            {exportLabel} <Icon iconType={BsBoxArrowUpRight} />
                        </NavButton>
                        <NavButton onClick={() => openAbout.current?.()}>
                            {about} <Icon iconType={BsInfoCircleFill} />
                        </NavButton>
                    </NavMore>
                </ul>
            </nav>
        </IconContext.Provider>
    );
};

export default Nav;
