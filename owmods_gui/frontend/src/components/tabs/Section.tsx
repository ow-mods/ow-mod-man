import { ReactNode } from "react";

export interface SectionProps {
    shown: boolean;
    children: ReactNode;
    className?: string;
}

const Section = (props: SectionProps) => {
    return (
        <div className={`section${props.shown ? " shown" : ""} ${props.className}`}>
            {props.children}
        </div>
    );
};

export default Section;
