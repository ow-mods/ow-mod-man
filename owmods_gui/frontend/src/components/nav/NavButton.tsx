import { ReactNode } from "react";

export interface NavButtonProps {
    children: ReactNode;
    ariaLabel?: string;
    onClick?: () => void;
}

export default (props: NavButtonProps) => {
    return (
        <li>
            <a
                className="fix-icons"
                aria-label={props.ariaLabel}
                onClick={() => props.onClick?.()}
                data-tooltip={props.ariaLabel}
                data-placement="bottom"
            >
                {props.children}
            </a>
        </li>
    );
};
