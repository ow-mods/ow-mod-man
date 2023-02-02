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
                    <NavButton labelPlacement="right" ariaLabel="Downloads">
                        <FaArrowDown />
                    </NavButton>
                    <NavButton labelPlacement="bottom" ariaLabel="Refresh">
                        <TbRefresh />
                    </NavButton>
                </ul>
                <ul>
                    <NavButton labelPlacement="bottom" ariaLabel="Run Game">
                        <FaPlay />
                    </NavButton>
                </ul>
                <ul>
                    <NavButton labelPlacement="bottom" ariaLabel="Help">
                        <FaQuestion />
                    </NavButton>
                    <NavButton labelPlacement="left" ariaLabel="More">
                        <HiDotsVertical />
                    </NavButton>
                </ul>
            </nav>
        </IconContext.Provider>
    );
};
