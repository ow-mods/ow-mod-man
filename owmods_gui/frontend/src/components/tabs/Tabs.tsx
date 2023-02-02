import { MdMonitor } from "react-icons/md";
import { FaGlobeAmericas, FaArrowAltCircleUp } from "react-icons/fa";

import Tab from "@components/tabs/Tab";
import Section from "./Section";
import { useState } from "react";
import LocalMods from "@components/mods/local/LocalMods";
import { IconContext } from "react-icons";
import RemoteMods from "@components/mods/remote/RemoteMods";

enum SectionType {
    Local,
    Remote,
    Updates
}

const Tabs = () => {
    const [shownSection, setShownSection] = useState(SectionType.Local);

    return (
        <>
            <IconContext.Provider value={{ className: "tab-icon" }}>
                <div className="grid tabs">
                    <Tab
                        selected={shownSection == SectionType.Local}
                        onClick={() => setShownSection(SectionType.Local)}
                    >
                        <MdMonitor /> Installed Mods
                    </Tab>
                    <Tab
                        selected={shownSection == SectionType.Remote}
                        onClick={() => setShownSection(SectionType.Remote)}
                    >
                        <FaGlobeAmericas /> Get Mods
                    </Tab>
                    <Tab
                        selected={shownSection == SectionType.Updates}
                        onClick={() => setShownSection(SectionType.Updates)}
                    >
                        <FaArrowAltCircleUp /> Updates
                    </Tab>
                </div>
            </IconContext.Provider>
            <div className="sections">
                <Section shown={shownSection == SectionType.Local}>
                    <LocalMods />
                </Section>
                <Section shown={shownSection == SectionType.Remote}>
                    <RemoteMods />
                </Section>
            </div>
        </>
    );
};

export default Tabs;
