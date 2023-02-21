import { MdMonitor } from "react-icons/md";
import { FaGlobeAmericas, FaArrowAltCircleUp } from "react-icons/fa";

import Tab from "@components/tabs/Tab";
import Section from "./Section";
import { memo, useState } from "react";
import LocalMods from "@components/mods/local/LocalMods";
import { IconContext } from "react-icons";
import RemoteMods from "@components/mods/remote/RemoteMods";
import Icon from "@components/Icon";
import { useTranslations } from "@hooks";
import UpdateMods from "@components/mods/updates/UpdateMods";

enum SectionType {
    Local,
    Remote,
    Updates
}

const Tabs = memo(() => {
    const [shownSection, setShownSection] = useState(SectionType.Local);

    const [installedMods, getMods, updates] = useTranslations([
        "INSTALLED_MODS",
        "GET_MODS",
        "UPDATES"
    ]);

    return (
        <>
            <IconContext.Provider value={{ className: "tab-icon" }}>
                <div className="grid tabs">
                    <Tab
                        selected={shownSection === SectionType.Local}
                        onClick={() => setShownSection(SectionType.Local)}
                    >
                        <Icon iconType={MdMonitor} label={installedMods} />
                    </Tab>
                    <Tab
                        selected={shownSection === SectionType.Remote}
                        onClick={() => setShownSection(SectionType.Remote)}
                    >
                        <Icon iconType={FaGlobeAmericas} label={getMods} />
                    </Tab>
                    <Tab
                        selected={shownSection === SectionType.Updates}
                        onClick={() => setShownSection(SectionType.Updates)}
                    >
                        <Icon iconType={FaArrowAltCircleUp} label={updates} />
                    </Tab>
                </div>
            </IconContext.Provider>
            <div className="sections">
                <Section shown={shownSection === SectionType.Local}>
                    <LocalMods />
                </Section>
                <Section className="remote" shown={shownSection === SectionType.Remote}>
                    <RemoteMods />
                </Section>
                <Section shown={shownSection === SectionType.Updates}>
                    <UpdateMods />
                </Section>
            </div>
        </>
    );
});

export default Tabs;
