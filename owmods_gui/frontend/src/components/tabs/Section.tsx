import { memo, ReactNode, useMemo } from "react";

export interface SectionProps {
    shown: boolean;
    children: ReactNode;
}

const Section = (props: SectionProps) => {
    return <div className={`section${props.shown ? " shown" : ""}`}>{props.children}</div>;
};

export default Section;
