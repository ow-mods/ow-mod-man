import { ReactNode } from "react";

export interface ModActionButtonProps {
    children: ReactNode;
    ariaLabel: string;
    onClick?: () => void;
}

const ModActionButton = (props: ModActionButtonProps) => {
    return (
        <a
            data-tooltip={props.ariaLabel}
            data-placement="left" /* Avoid letting the tooltips go out of the window */
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

export default ModActionButton;
