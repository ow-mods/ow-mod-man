import { CSSProperties } from "react";

export interface SpinnerProps {
    className?: string;
    style?: CSSProperties;
    children?: string;
}

const Spinner = (props: SpinnerProps) => {
    return (
        <p style={props.style} aria-busy={true} className={props.className}>
            {props.children ?? ""}
        </p>
    );
};

export default Spinner;
