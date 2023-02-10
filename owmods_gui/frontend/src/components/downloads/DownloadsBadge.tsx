import { useTauriCount } from "@hooks";
import { memo } from "react";

const DownloadsBadge = memo(() => {
    const count = useTauriCount("INSTALL-START", "INSTALL-FINISH");

    return <div className={`download-badge${count === 0 ? " d-none" : ""}`}>{count}</div>;
});

export default DownloadsBadge;
