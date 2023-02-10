import { useTranslation } from "@hooks";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useRef, useState } from "react";

interface ActiveDownloadProps {
    id: string;
    progressAction: "Download" | "Extract" | "Wine";
    progressType: "Definite" | "Indefinite";
    message: string;
    progress: number;
    len: number;
}

interface ProgressIncrementPayload {
    increment: {
        id: string;
        amount: number;
    };
}

interface ProgressMessagePayload {
    changeMsg: {
        id: string;
        newMsg: string;
    };
}

interface ProgressFinishPayload {
    finish: {
        id: string;
        msg: string;
    };
}

const ActiveDownload = (props: ActiveDownloadProps) => {
    return (
        <div className="downloads-row">
            <p className="download-header">{props.message}</p>
            <progress
                value={
                    props.progressType === "Indefinite" && props.progress === 0
                        ? undefined
                        : (props.progress / props.len) * 100
                }
                max={100}
            />
        </div>
    );
};

const DownloadsPopout = () => {
    const [downloads, setDownloads] = useState<Record<string, ActiveDownloadProps>>({});

    const downloadsRef = useRef(downloads);

    useEffect(() => {
        downloadsRef.current = downloads;
    }, [downloads]);

    useEffect(() => {
        listen("PROGRESS-START", (p) => {
            const data = p.payload as ActiveDownloadProps;
            if (data.progressAction !== "Wine") {
                console.debug(p.payload);
                if (data.id in downloadsRef.current) {
                    delete downloadsRef.current[data.id];
                }
                setDownloads({ ...downloadsRef.current, [data.id]: { ...data, progress: 0 } });
            }
        }).catch(console.warn);
        listen("PROGRESS-INCREMENT", (e) => {
            const payload = (e.payload as ProgressIncrementPayload).increment;
            const current = downloadsRef.current[payload.id];
            if (current) {
                current.progress += payload.amount;
                setDownloads({ ...downloadsRef.current, [current.id]: current });
            }
        });
        listen("PROGRESS-MSG", (e) => {
            const payload = (e.payload as ProgressMessagePayload).changeMsg;
            const current = downloadsRef.current[payload.id];
            if (current) {
                console.debug(payload);
                current.message = payload.newMsg;
                setDownloads({ ...downloadsRef.current, [current.id]: current });
            }
        });
        listen("PROGRESS-FINISH", (e) => {
            const payload = (e.payload as ProgressFinishPayload).finish;
            const current = downloadsRef.current[payload.id];
            if (current && current.progressAction === "Extract") {
                current.message = payload.msg;
                if (current.progressType === "Indefinite") {
                    current.progress = 1;
                }
                setDownloads({ ...downloadsRef.current, [current.id]: current });
            }
        });
    }, []);

    const noDownloads = useTranslation("NO_DOWNLOADS");

    return (
        <div className="downloads-popout">
            {Object.keys(downloads).length === 0 ? (
                <p className="no-downloads">{noDownloads}</p>
            ) : (
                <>
                    <a
                        role="button"
                        href="#"
                        className="clear-downloads secondary"
                        onClick={() => setDownloads({})}
                    >
                        Clear All
                    </a>
                    {Object.values(downloads)
                        .reverse()
                        .map((d) => (
                            <ActiveDownload key={d.id} {...d} />
                        ))}
                </>
            )}
        </div>
    );
};

export default DownloadsPopout;
