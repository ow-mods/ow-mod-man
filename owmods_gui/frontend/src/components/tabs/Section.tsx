import { ReactNode, memo } from "react";

export interface SectionProps {
    shown: boolean;
    children: ReactNode;
    className?: string;
}

const Section = memo(function Section(props: SectionProps) {
    return (
        <div className={`section${props.shown ? " shown" : ""} ${props.className ?? ""}`}>
            {props.children}
        </div>
    );
});

export default Section;
