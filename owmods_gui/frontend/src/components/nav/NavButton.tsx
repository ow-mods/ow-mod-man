import { ReactNode } from "react";

export interface NavButtonProps {
    children: ReactNode;
    labelPlacement?: string;
    ariaLabel?: string;
    onClick?: () => void;
    disabled?: boolean;
    className?: string;
}

const NavButton = (props: NavButtonProps) => {
    return (
        <li>
            <a
                className={`fix-icons ${props.className ?? ""}`}
                aria-label={props.ariaLabel}
                onClick={() => {
                    if (!props.disabled) props.onClick?.();
                }}
                data-tooltip={props.ariaLabel}
                data-placement={props.labelPlacement}
            >
                {props.children}
            </a>
        </li>
    );
};

export default NavButton;
