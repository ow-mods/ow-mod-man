import { hooks } from "@commands";
import { memo } from "react";

const DownloadsBadge = memo(function DownloadsBadge() {
    const count = Object.values(hooks.getDownloads("PROGRESS-UPDATE")[1] ?? {}).filter(
        (d) => d.success === null
    ).length;
    return <div className={`download-badge${count === 0 ? " d-none" : ""}`}>{count}</div>;
});

export default DownloadsBadge;
