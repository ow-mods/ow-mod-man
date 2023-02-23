import { FaPlay, FaQuestion, FaCog, FaInfoCircle } from "react-icons/fa";
import { TbRefresh } from "react-icons/tb";
import { RiInstallFill } from "react-icons/ri";
import { RxActivityLog } from "react-icons/rx";

import NavButton from "@components/nav/NavButton";
import { IconContext } from "react-icons";
import Icon from "@components/Icon";
import NavMore from "./NavMore";
import { useCallback, useRef } from "react";
import SettingsModal from "@components/modals/SettingsModal";
import InstallFromModal from "@components/modals/InstallFromModal";
import AboutModal from "@components/modals/AboutModal";
import Downloads from "../downloads/Downloads";
import { useTranslations } from "@hooks";
import { commands } from "@commands";

const Nav = () => {
    const openSettings = useRef<() => void>(() => null);
    const openInstallFrom = useRef<() => void>(() => null);
    const openAbout = useRef<() => void>(() => null);

    const onRefresh = useCallback(() => {
        commands.refreshLocalDb().catch((e) => console.warn(e));
        commands.refreshRemoteDb().catch((e) => console.warn(e));
    }, []);

    const [refresh, runGame, help, settings, installFrom, about, logs] = useTranslations([
        "REFRESH",
        "RUN_GAME",
        "HELP",
        "SETTINGS",
        "INSTALL_FROM",
        "ABOUT",
        "LOGS"
    ]);

    return (
        <IconContext.Provider value={{ className: "nav-icon" }}>
            <SettingsModal open={openSettings} />
            <InstallFromModal open={openInstallFrom} />
            <AboutModal open={openAbout} />
            <nav>
                <ul>
                    <Downloads />
                    <NavButton onClick={onRefresh} labelPlacement="bottom" ariaLabel={refresh}>
                        <Icon iconType={TbRefresh} />
                    </NavButton>
                </ul>
                <ul>
                    <NavButton labelPlacement="bottom" ariaLabel={runGame}>
                        <Icon iconClassName="main-icon" iconType={FaPlay} />
                    </NavButton>
                </ul>
                <ul>
                    <NavButton labelPlacement="bottom" ariaLabel={help}>
                        <Icon iconType={FaQuestion} />
                    </NavButton>
                    <NavMore>
                        {/* Dropdown uses RTL */}
                        <NavButton onClick={() => openSettings.current?.()}>
                            {settings} <Icon iconType={FaCog} />
                        </NavButton>
                        <NavButton onClick={() => openInstallFrom.current?.()}>
                            ...{installFrom} <Icon iconType={RiInstallFill} />
                        </NavButton>
                        <NavButton onClick={() => openAbout.current?.()}>
                            {about} <Icon iconType={FaInfoCircle} />
                        </NavButton>
                        <NavButton>
                            {logs} <Icon iconType={RxActivityLog} />
                        </NavButton>
                    </NavMore>
                </ul>
            </nav>
        </IconContext.Provider>
    );
};

export default Nav;
