import { ReactNode } from "react";

export interface TabProps {
    selected: boolean;
    children: ReactNode;
    onClick?: () => void;
}

export default (props: TabProps) => {
    return (
        <div onClick={() => props.onClick?.()} className={`tab${props.selected ? " shown" : ""}`}>
            <div className="fix-icons">{props.children}</div>
        </div>
    );
};
