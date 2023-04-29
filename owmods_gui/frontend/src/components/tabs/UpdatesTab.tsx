import { hooks } from "@commands";
import Icon from "@components/common/Icon";
import { useGetTranslation } from "@hooks";
import { BsArrowUpCircle, BsArrowUpCircleFill } from "react-icons/bs";
import Tab, { TabProps } from "./Tab";

const UpdatesTab = (props: Omit<TabProps, "children">) => {
    const updatesList = hooks.getUpdatableMods(["LOCAL-REFRESH", "REMOTE-REFRESH"])[1];
    const getTranslation = useGetTranslation();

    const count = updatesList?.length ?? 0;
    const countLabel = count === 0 ? "" : `(${updatesList?.length})`;

    return (
        <Tab {...props}>
            <Icon
                iconType={count === 0 ? BsArrowUpCircle : BsArrowUpCircleFill}
                label={getTranslation("UPDATES", { amount: countLabel })}
            />
        </Tab>
    );
};

export default UpdatesTab;
