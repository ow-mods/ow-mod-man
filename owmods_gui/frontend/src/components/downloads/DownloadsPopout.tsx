import Icon from "@components/Icon";
import { useTranslations } from "@hooks";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useRef, useState } from "react";
import { BsCheck2, BsTrashFill } from "react-icons/bs";

interface ActiveDownloadProps {
    id: string;
    progressAction: "Download" | "Extract" | "Wine";
    progressType: "Definite" | "Indefinite";
    msg: string;
    progress: number;
    len: number;
}

interface ProgressIncrementPayload {
    id: string;
    progress: number;
}

interface ProgressMessagePayload {
    id: string;
    msg: string;
}

interface ProgressFinishPayload {
    id: string;
    msg: string;
}

const ActiveDownload = (props: ActiveDownloadProps) => {
    return (
        <div
            className={`downloads-row${
                props.progressAction === "Extract" && props.progress >= props.len
                    ? " download-done"
                    : ""
            }`}
        >
            <p className="download-header fix-icons">
                <Icon iconType={BsCheck2} /> {props.msg}
            </p>
            <progress
                value={
                    props.progressType === "Indefinite" && props.progress === 0
                        ? undefined
                        : props.progress
                }
                max={props.len}
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
        // Necessary evil, can't unsubscribe bc everything is async
        let cancelled = false;
        listen("PROGRESS-START", (p) => {
            if (cancelled) return;
            const data = p.payload as ActiveDownloadProps;
            if (data.progressAction !== "Wine") {
                if (data.id in downloadsRef.current) {
                    delete downloadsRef.current[data.id];
                }
                setDownloads({ ...downloadsRef.current, [data.id]: { ...data, progress: 0 } });
            }
        }).catch(console.warn);
        listen("PROGRESS-INCREMENT", (e) => {
            if (cancelled) return;
            const payload = e.payload as ProgressIncrementPayload;
            const current = downloadsRef.current[payload.id];
            if (current) {
                current.progress = payload.progress;
                setDownloads({ ...downloadsRef.current, [current.id]: current });
            }
        }).catch(console.warn);
        listen("PROGRESS-MESSAGE", (e) => {
            if (cancelled) return;
            const payload = e.payload as ProgressMessagePayload;
            const current = downloadsRef.current[payload.id];
            if (current) {
                current.msg = payload.msg;
                setDownloads({ ...downloadsRef.current, [current.id]: current });
            }
        }).catch(console.warn);
        listen("PROGRESS-FINISH", (e) => {
            if (cancelled) return;
            const payload = e.payload as ProgressFinishPayload;
            const current = downloadsRef.current[payload.id];
            if (current && current.progressAction === "Extract") {
                current.msg = payload.msg;
                setDownloads({ ...downloadsRef.current, [current.id]: current });
            }
        }).catch(console.warn);
        return () => {
            cancelled = true;
        };
    }, []);

    const [noDownloads, clearDownloads] = useTranslations(["NO_DOWNLOADS", "CLEAR_DOWNLOADS"]);

    return (
        <div className="downloads-popout">
            <header>Downloads</header>
            {Object.keys(downloads).length === 0 ? (
                <p className="no-downloads">{noDownloads}</p>
            ) : (
                <>
                    <a
                        href="#"
                        className="clear-downloads"
                        data-tooltip={clearDownloads}
                        data-placement="left"
                        onClick={() => setDownloads({})}
                    >
                        <Icon iconType={BsTrashFill} />
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
