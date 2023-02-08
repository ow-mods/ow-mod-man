import { FaPlay, FaQuestion, FaCog, FaInfoCircle } from "react-icons/fa";
import { TbRefresh } from "react-icons/tb";
import { RiInstallFill } from "react-icons/ri";
import { RxActivityLog } from "react-icons/rx";

import NavButton from "@components/nav/NavButton";
import { IconContext } from "react-icons";
import Icon from "@components/Icon";
import NavMore from "./NavMore";
import { useRef } from "react";
import SettingsModal from "@components/modals/SettingsModal";
import InstallFromModal from "@components/modals/InstallFromModal";
import AboutModal from "@components/modals/AboutModal";
import Downloads from "../downloads/Downloads";

const Nav = () => {
    const openSettings = useRef<() => void>(() => null);
    const openInstallFrom = useRef<() => void>(() => null);
    const openAbout = useRef<() => void>(() => null);

    return (
        <IconContext.Provider value={{ className: "nav-icon" }}>
            <SettingsModal open={openSettings} />
            <InstallFromModal open={openInstallFrom} />
            <AboutModal open={openAbout} />
            <nav>
                <ul>
                    <Downloads />
                    <NavButton labelPlacement="bottom" ariaLabel="Refresh">
                        <Icon iconType={TbRefresh} />
                    </NavButton>
                </ul>
                <ul>
                    <NavButton labelPlacement="bottom" ariaLabel="Run Game">
                        <Icon iconClassName="main-icon" iconType={FaPlay} />
                    </NavButton>
                </ul>
                <ul>
                    <NavButton labelPlacement="bottom" ariaLabel="Help">
                        <Icon iconType={FaQuestion} />
                    </NavButton>
                    <NavMore>
                        {/* Dropdown uses RTL */}
                        <NavButton onClick={() => openSettings.current?.()}>
                            Settings <Icon iconType={FaCog} />
                        </NavButton>
                        <NavButton onClick={() => openInstallFrom.current?.()}>
                            ...Install From <Icon iconType={RiInstallFill} />
                        </NavButton>
                        <NavButton onClick={() => openAbout.current?.()}>
                            About <Icon iconType={FaInfoCircle} />
                        </NavButton>
                        <NavButton>
                            Logs <Icon iconType={RxActivityLog} />
                        </NavButton>
                    </NavMore>
                </ul>
            </nav>
        </IconContext.Provider>
    );
};

export default Nav;
