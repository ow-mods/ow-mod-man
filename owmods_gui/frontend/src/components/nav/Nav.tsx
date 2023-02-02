import { FaPlay, FaQuestion, FaArrowDown, FaCog } from "react-icons/fa";
import { TbRefresh } from "react-icons/tb";
import { HiDotsVertical } from "react-icons/hi";

import NavButton from "@components/nav/NavButton";
import { IconContext } from "react-icons";

export default () => {
    return (
        <IconContext.Provider value={{ className: "nav-icon" }}>
            <nav>
                <ul>
                    <NavButton ariaLabel="Downloads">
                        <FaArrowDown />
                    </NavButton>
                    <NavButton ariaLabel="Refresh">
                        <TbRefresh />
                    </NavButton>
                </ul>
                <ul>
                    <NavButton ariaLabel="Run Game">
                        <FaPlay />
                    </NavButton>
                </ul>
                <ul>
                    <NavButton ariaLabel="Help">
                        <FaQuestion />
                    </NavButton>
                    <NavButton ariaLabel="More">
                        <HiDotsVertical />
                    </NavButton>
                </ul>
            </nav>
        </IconContext.Provider>
    );
};
