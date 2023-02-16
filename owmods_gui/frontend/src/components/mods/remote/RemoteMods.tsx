import { useTauri } from "@hooks";
import { memo } from "react";
import AutoSizer from "react-virtualized-auto-sizer";
import { FixedSizeList } from "react-window";
import RemoteModRow from "./RemoteModRow";

const RemoteMods = memo(() => {
    const [status, mods, err] = useTauri<string[]>("REMOTE-REFRESH", "get_remote_mods");

    if (status === "Loading" && mods === null) {
        return <div className="mod-list center-loading" aria-busy></div>;
    } else if (status === "Error") {
        return <p className="mod-list center-loading">{err!.toString()}</p>;
    } else {
        const remote_mods = mods!;
        return (
            <AutoSizer>
                {({ width, height }) => (
                    <FixedSizeList
                        itemCount={remote_mods.length}
                        itemSize={120}
                        itemKey={(index) => remote_mods[index]}
                        width={width}
                        height={height}
                        className="mod-list remote"
                    >
                        {({ index, style }) => (
                            <RemoteModRow style={style} uniqueName={remote_mods[index]} />
                        )}
                    </FixedSizeList>
                )}
            </AutoSizer>
        );
    }
});

export default RemoteMods;
