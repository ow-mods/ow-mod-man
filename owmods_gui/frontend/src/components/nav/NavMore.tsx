import Icon from "@components/Icon";
import { ReactNode } from "react";
import { HiDotsVertical } from "react-icons/hi";
import NavButton from "./NavButton";

export interface NavMoreProps {
    children: ReactNode;
}

const NavMore = (props: NavMoreProps) => (
    <li>
        <details role="list" dir="rtl">
            <summary>
                <NavButton ariaLabel="More" labelPlacement="left">
                    <Icon iconType={HiDotsVertical} />
                </NavButton>
            </summary>
            <ul role="listbox">{props.children}</ul>
        </details>
    </li>
);

export default NavMore;
