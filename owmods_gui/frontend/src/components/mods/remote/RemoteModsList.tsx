import { hooks } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import { memo } from "react";
import RemoteModRow from "./RemoteModRow";
import { Virtuoso } from "react-virtuoso";

export interface RemoteModsListProps {
    filter: string;
}

const RemoteModsList = memo(function RemoteModsList(props: RemoteModsListProps) {
    const [status, mods, err] = hooks.getRemoteMods("REMOTE-REFRESH", { filter: props.filter });

    if (status === "Loading" && mods === null) {
        return <CenteredSpinner />;
    } else if (status === "Error") {
        return <p className="mod-list center">{err!.toString()}</p>;
    } else {
        return (
            <Virtuoso
                increaseViewportBy={5000}
                computeItemKey={(index) => `${index}-${mods![index]}`}
                className="mod-list remote"
                data={mods!}
                itemContent={(_, uniqueName) => <RemoteModRow uniqueName={uniqueName} />}
            />
        );
    }
});

export default RemoteModsList;
