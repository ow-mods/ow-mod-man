import { hooks } from "@commands";
import Icon from "@components/Icon";
import { useTranslation } from "@hooks";
import { BsArrowUpCircle, BsArrowUpCircleFill } from "react-icons/bs";
import Tab, { TabProps } from "./Tab";

const UpdatesTab = (props: Omit<TabProps, "children">) => {
    const updatesList = hooks.getUpdatableMods(["LOCAL-REFRESH", "REMOTE-REFRESH"])[1];

    const count = updatesList?.length ?? 0;
    const countLabel = count === 0 ? "" : `(${updatesList?.length})`;
    const updatesLabel = useTranslation("UPDATES", { amount: countLabel });

    return (
        <Tab {...props}>
            <Icon
                iconType={count === 0 ? BsArrowUpCircle : BsArrowUpCircleFill}
                label={updatesLabel}
            />
        </Tab>
    );
};

export default UpdatesTab;
