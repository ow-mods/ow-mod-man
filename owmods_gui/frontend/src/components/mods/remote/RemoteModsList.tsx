import { hooks } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import { useVirtualizer } from "@tanstack/react-virtual";
import { memo, useLayoutEffect, useRef, useState } from "react";
import RemoteModRow from "./RemoteModRow";
import ItemMeasurer from "@components/common/ItemMeasurer";

export interface RemoteModsListProps {
    filter: string;
}

const RemoteModsList = memo((props: RemoteModsListProps) => {
    const currentPos = useRef(0);
    const [isHidden, setIsHidden] = useState(true);
    const parentRef = useRef<HTMLDivElement | null>(null);
    const [status, mods, err] = hooks.getRemoteMods("REMOTE-REFRESH", { filter: props.filter });
    const modsVirtualizer = useVirtualizer({
        count: mods?.length ?? 0,
        overscan: 35,
        getScrollElement: () => parentRef.current,
        getItemKey: (index) => `${index}-${mods ? mods[index] : ""}`,
        estimateSize: () => 65
    });

    const items = modsVirtualizer.getVirtualItems();

    const elHeight = modsVirtualizer.scrollElement?.offsetHeight ?? -1;

    useLayoutEffect(() => {
        if (elHeight === 0) {
            setIsHidden(true);
        } else {
            setTimeout(() => {
                modsVirtualizer.scrollToOffset(currentPos.current, {
                    align: "start",
                    behavior: "auto"
                });
                setTimeout(() => setIsHidden(false), 50);
            }, 100);
        }
    }, [elHeight]);

    if (status === "Loading" && mods === null) {
        return <CenteredSpinner />;
    } else if (status === "Error") {
        return <p className="mod-list center">{err!.toString()}</p>;
    } else {
        return (
            <div
                style={{ visibility: isHidden ? "hidden" : "visible" }}
                ref={parentRef}
                className="mod-list remote"
            >
                <div
                    style={{
                        height: modsVirtualizer.getTotalSize()
                    }}
                >
                    <div
                        style={{
                            translate: `translateY(${items[0]?.start ?? 30}px)`
                        }}
                    >
                        {items.map((item) => (
                            <ItemMeasurer
                                hideOnMeasure
                                className="mod-row remote"
                                index={item.index}
                                start={item.start}
                                measure={modsVirtualizer.measureElement}
                                as={RemoteModRow}
                                key={item.key}
                                uniqueName={mods![item.index]}
                            />
                        ))}
                    </div>
                </div>
            </div>
        );
    }
});

export default RemoteModsList;
