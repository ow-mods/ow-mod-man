import { ReactNode } from "react";

export interface TabProps {
    children: ReactNode;
    hash: string;
}

export default (props: TabProps) => {
    const onClick = () => {
        window.location.hash = props.hash;
    };

    return (
        <div onClick={onClick} id={props.hash} className="tab">
            <div className="fix-icons">{props.children}</div>
        </div>
    );
};
