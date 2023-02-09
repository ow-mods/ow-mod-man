import { useTranslation } from "@hooks";
import { ReactNode } from "react";

export interface ModHeaderProps {
    children: ReactNode;
    name: string;
    author: string;
}

const ModHeader = (props: ModHeaderProps) => {
    const by = useTranslation("BY");

    return (
        <summary className="mod-header">
            <div className="mod-heading">
                <span className="mod-name">{props.name}</span>
                <small className="mod-authors">
                    {by} {props.author}
                </small>
            </div>
            <div className="mod-actions">{props.children}</div>
        </summary>
    );
};

export default ModHeader;
