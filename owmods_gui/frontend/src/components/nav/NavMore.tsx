import Icon from "@components/common/Icon";
import { useTranslation } from "@hooks";
import { ReactNode, useRef } from "react";
import { HiDotsVertical } from "react-icons/hi";
import NavButton from "./NavButton";

export interface NavMoreProps {
    children: ReactNode;
}

const NavMore = (props: NavMoreProps) => {
    const more = useTranslation("MORE");
    const detailsRef = useRef<HTMLDetailsElement>(null);

    return (
        <li>
            <details ref={detailsRef} role="list" dir="rtl">
                <summary>
                    <NavButton ariaLabel={more} labelPlacement="left">
                        <Icon iconType={HiDotsVertical} />
                    </NavButton>
                </summary>
                <ul
                    onClick={() => {
                        if (!detailsRef.current) return;
                        detailsRef.current.open = false;
                    }}
                    role="listbox"
                >
                    {props.children}
                </ul>
            </details>
        </li>
    );
};

export default NavMore;
