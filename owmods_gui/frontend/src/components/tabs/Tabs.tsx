import { MdMonitor } from "react-icons/md";
import { FaGlobeAmericas, FaArrowAltCircleUp } from "react-icons/fa";

import Tab from "@components/tabs/Tab";

export default () => {
    return (
        <div className="grid tabs">
            <Tab hash="local">
                <MdMonitor /> Installed Mods
            </Tab>
            <Tab hash="remote">
                <FaGlobeAmericas /> Get Mods
            </Tab>
            <Tab hash="updates">
                <FaArrowAltCircleUp /> Updates
            </Tab>
        </div>
    );
};
