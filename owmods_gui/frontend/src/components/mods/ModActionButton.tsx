import { ReactNode } from "react";

export interface ModActionButtonProps {
    children: ReactNode;
    ariaLabel: string;
    onClick?: () => void;
}

export default (props: ModActionButtonProps) => {
    return (
        <a
            data-tooltip={props.ariaLabel}
            data-placement="top"
            className="fix-icons"
            onClick={(e) => {
                e.preventDefault();
                props.onClick?.();
            }}
            aria-label={props.ariaLabel}
        >
            {props.children}
        </a>
    );
};
