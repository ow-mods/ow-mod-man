import { useState } from "react";

interface ActiveDownloadProps {
    id: string;
}

interface DownloadPayload {
    id: string;
    progress?: number;
    message: string;
}

const ActiveDownload = (props: ActiveDownloadProps) => {
    // Temp state for rn, will use tauri later.
    const progress = useState<DownloadPayload>({
        id: props.id,
        message: "Downloading xen.NewHorizons"
    })[0];

    return (
        <div className="downloads-row">
            <p className="download-header">{progress.message}</p>
            <progress value={progress.progress} />
        </div>
    );
};

const DownloadsPopout = () => {
    const downloads: ActiveDownloadProps[] = [];

    return (
        <div className="downloads-popout">
            {downloads.length === 0 ? (
                <p className="no-downloads">No Downloads</p>
            ) : (
                downloads.map((d) => <ActiveDownload key={d.id} {...d} />)
            )}
        </div>
    );
};

export default DownloadsPopout;
