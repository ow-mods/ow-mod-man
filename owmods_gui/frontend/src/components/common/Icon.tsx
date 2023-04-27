import { memo } from "react";
import { IconType } from "react-icons";

export interface IconProps {
    iconType: IconType;
    label?: string;
    iconClassName?: string;
}

// "Pure" icon component, use to prevent expensive rerenders
const Icon = memo(
    function Icon(props: IconProps) {
        return (
            <>
                {props.iconType({ className: props.iconClassName })}
                {props.label}
            </>
        );
    },
    (prev, next) => prev.label === next.label
);

export default Icon;
