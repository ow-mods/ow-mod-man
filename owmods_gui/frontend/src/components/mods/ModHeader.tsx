import { ReactNode } from "react";
import { IconContext } from "react-icons";

export interface ModHeaderProps {
    children: ReactNode;
    name: string;
    authors: string;
}

export default (props: ModHeaderProps) => {
    return (
        <summary className="mod-header">
            <div className="mod-heading">
                <span className="mod-name">{props.name}</span>
                <small className="mod-authors">by {props.authors}</small>
            </div>
            <div className="mod-actions">{props.children}</div>
        </summary>
    );
};
