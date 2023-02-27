import Icon from "@components/common/Icon";
import { useTranslation } from "@hooks";
import { ReactNode } from "react";
import { HiDotsVertical } from "react-icons/hi";
import NavButton from "./NavButton";

export interface NavMoreProps {
    children: ReactNode;
}

const NavMore = (props: NavMoreProps) => {
    const more = useTranslation("MORE");

    return (
        <li>
            <details role="list" dir="rtl">
                <summary>
                    <NavButton ariaLabel={more} labelPlacement="left">
                        <Icon iconType={HiDotsVertical} />
                    </NavButton>
                </summary>
                <ul role="listbox">{props.children}</ul>
            </details>
        </li>
    );
};

export default NavMore;
