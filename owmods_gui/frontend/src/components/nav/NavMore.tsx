import Icon from "@components/common/Icon";
import { useTranslation } from "@hooks";
import { ReactNode, useState } from "react";
import { HiDotsVertical } from "react-icons/hi";
import NavButton from "./NavButton";

export interface NavMoreProps {
    children: ReactNode;
}

const NavMore = (props: NavMoreProps) => {
    const more = useTranslation("MORE");
    const [open, setOpen] = useState(false);

    return (
        <li>
            <details open={open} role="list" dir="rtl">
                <summary
                    onClick={(e) => {
                        e.preventDefault();
                        setOpen(true);
                    }}
                >
                    <NavButton ariaLabel={more} labelPlacement="left">
                        <Icon iconType={HiDotsVertical} />
                    </NavButton>
                </summary>
                <ul onClick={() => setOpen(false)} role="listbox">
                    {props.children}
                </ul>
            </details>
        </li>
    );
};

export default NavMore;
