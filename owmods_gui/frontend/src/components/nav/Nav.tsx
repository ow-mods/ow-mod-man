import {
    BsPlayFill,
    BsQuestion,
    BsGearFill,
    BsInfoCircleFill,
    BsArrowRepeat,
    BsBoxArrowInDown,
    BsFilterLeft
} from "react-icons/bs";

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
                        <Icon iconType={BsArrowRepeat} />
                    </NavButton>
                </ul>
                <ul>
                    <NavButton labelPlacement="bottom" ariaLabel={runGame}>
                        <Icon iconClassName="main-icon" iconType={BsPlayFill} />
                    </NavButton>
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
                        <NavButton onClick={() => openAbout.current?.()}>
                            {about} <Icon iconType={BsInfoCircleFill} />
                        </NavButton>
                        <NavButton>
                            {logs} <Icon iconType={BsFilterLeft} />
                        </NavButton>
                    </NavMore>
                </ul>
            </nav>
        </IconContext.Provider>
    );
};

export default Nav;
