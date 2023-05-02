import { BsDisplay, BsGlobe } from "react-icons/bs";

import Tab from "@components/tabs/Tab";
import Section from "./Section";
import { memo, useState } from "react";
import LocalMods from "@components/mods/local/LocalMods";
import { IconContext } from "react-icons";
import RemoteMods from "@components/mods/remote/RemoteMods";
import Icon from "@components/common/Icon";
import { useGetTranslation } from "@hooks";
import UpdateMods from "@components/mods/updates/UpdateMods";
import UpdatesTab from "./UpdatesTab";

enum SectionType {
    Local,
    Remote,
    Updates
}

const Tabs = memo(function Tabs() {
    const [shownSection, setShownSection] = useState(SectionType.Local);
    const getTranslation = useGetTranslation();

    return (
        <>
            <IconContext.Provider value={{ className: "tab-icon" }}>
                <div className="tabs-wrapper">
                    <div className="grid tabs max-width">
                        <Tab
                            selected={shownSection === SectionType.Local}
                            onClick={() => setShownSection(SectionType.Local)}
                        >
                            <Icon iconType={BsDisplay} label={getTranslation("INSTALLED_MODS")} />
                        </Tab>
                        <Tab
                            selected={shownSection === SectionType.Remote}
                            onClick={() => setShownSection(SectionType.Remote)}
                        >
                            <Icon iconType={BsGlobe} label={getTranslation("GET_MODS")} />
                        </Tab>
                        <UpdatesTab
                            selected={shownSection === SectionType.Updates}
                            onClick={() => setShownSection(SectionType.Updates)}
                        />
                    </div>
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
