import Icon from "@components/common/Icon";
import { useGetTranslation } from "@hooks";
import { ReactNode, useRef } from "react";
import { HiDotsVertical } from "react-icons/hi";
import NavButton from "./NavButton";

export interface NavMoreProps {
    children: ReactNode;
}

const NavMore = (props: NavMoreProps) => {
    const getTranslation = useGetTranslation();
    const detailsRef = useRef<HTMLDetailsElement>(null);

    return (
        <li>
            <details ref={detailsRef} role="list" dir="rtl">
                <summary>
                    <NavButton ariaLabel={getTranslation("MORE")} labelPlacement="left">
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
