import { ComponentClass, FunctionComponent, ReactNode } from "react";
import { useCallback, useLayoutEffect, useRef } from "react";

export interface ItemMeasurerProps {
    index: number;
    start: number;
    measure: (node: Element | null) => void;
    as: FunctionComponent<any> | ComponentClass<any> | string;
    children?: ReactNode;
    className?: string;
    hideOnMeasure?: boolean;
    [rest: string]: any;
}

const ItemMeasurer = ({
    index,
    start,
    measure,
    className,
    children,
    as,
    ...restProps
}: ItemMeasurerProps) => {
    const elRef = useRef<Element | null>(null);

    const measureRef = useRef(measure);
    measureRef.current = measure;

    const refSetter = useCallback((el: Element | null) => {
        elRef.current = el;
    }, []);

    useLayoutEffect(() => {
        measureRef.current(elRef.current);
    }, []);

    const Tag = as;

    return (
        <div
            style={{ position: "absolute", top: start }}
            className={className}
            ref={refSetter}
            data-index={index}
        >
            <Tag {...restProps}>{children}</Tag>
        </div>
    );
};

export default ItemMeasurer;
