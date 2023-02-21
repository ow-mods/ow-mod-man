import { ReactNode } from "react";

export interface ModHeaderProps {
    children: ReactNode;
    name: string;
    subtitle: string;
}

const ModHeader = (props: ModHeaderProps) => {
    return (
        <summary className="mod-header">
            <div className="mod-heading">
                <span className="mod-name">{props.name}</span>
                <small className="mod-authors">{props.subtitle}</small>
            </div>
            <div className="mod-actions">{props.children}</div>
        </summary>
    );
};

export default ModHeader;
