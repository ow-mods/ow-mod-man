import { commands } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import { useGetTranslation } from "@hooks";
import { memo, useEffect, useRef, useState } from "react";
import RemoteModsList from "./RemoteModsList";

const RemoteMods = memo(function RemoteMods() {
    const [filter, setFilter] = useState("");
    const [tempFilter, setTempFilter] = useState("");
    const getTranslation = useGetTranslation();

    useEffect(() => {
        commands.refreshRemoteDb();
    }, []);

    const activeTimeout = useRef<number | null>(null);

    const onChangeFilter = (newFilter: string) => {
        setTempFilter(newFilter);
        if (activeTimeout.current) {
            clearTimeout(activeTimeout.current);
        }
        activeTimeout.current = setTimeout(() => setFilter(newFilter), 450);
    };

    return (
        <>
            <input
                placeholder={getTranslation("SEARCH")}
                className="remote-mod-search"
                id="searchRemote"
                value={tempFilter}
                onChange={(e) => onChangeFilter(e.target.value)}
            />
            {tempFilter !== filter ? <CenteredSpinner /> : <RemoteModsList filter={filter} />}
        </>
    );
});

export default RemoteMods;
