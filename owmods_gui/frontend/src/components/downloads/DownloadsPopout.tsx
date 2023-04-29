import Icon from "@components/common/Icon";
import { useGetTranslation } from "@hooks";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useRef, useState } from "react";
import { BsCheck2, BsTrashFill, BsXCircleFill } from "react-icons/bs";

interface ActiveDownloadProps {
    id: string;
    progressAction: "Download" | "Extract" | "Wine";
    progressType: "Definite" | "Indefinite";
    failed: boolean;
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
    success: boolean;
    msg: string;
}

const ActiveDownload = (props: ActiveDownloadProps) => {
    const done = props.progress >= props.len && props.progressAction === "Extract";

    return (
        <div
            className={`downloads-row${
                props.failed ? " download-failed" : done ? " download-done" : ""
            }`}
        >
            <p className="download-header fix-icons">
                <Icon iconClassName="download-icon-failure" iconType={BsXCircleFill} />
                <Icon iconClassName="download-icon-success" iconType={BsCheck2} /> {props.msg}
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
    const getTranslation = useGetTranslation();

    const downloadsRef = useRef(downloads);

    useEffect(() => {
        let cancelled = false;
        listen("PROGRESS-START", (p) => {
            if (cancelled) return;
            const data = p.payload as ActiveDownloadProps;
            if (data.progressAction !== "Wine") {
                if (data.id in downloadsRef.current) {
                    delete downloadsRef.current[data.id];
                }
                downloadsRef.current = {
                    ...downloadsRef.current,
                    [data.id]: { ...data, progress: 0 }
                };
                setDownloads(downloadsRef.current);
            }
        }).catch(console.warn);
        listen("PROGRESS-INCREMENT", (e) => {
            if (cancelled) return;
            const payload = e.payload as ProgressIncrementPayload;
            const current = downloadsRef.current[payload.id];
            if (current) {
                current.progress = payload.progress;
                downloadsRef.current = { ...downloadsRef.current, [current.id]: current };
                setDownloads(downloadsRef.current);
            }
        }).catch(console.warn);
        listen("PROGRESS-MESSAGE", (e) => {
            if (cancelled) return;
            const payload = e.payload as ProgressMessagePayload;
            const current = downloadsRef.current[payload.id];
            if (current) {
                current.msg = payload.msg;
                downloadsRef.current = { ...downloadsRef.current, [current.id]: current };
                setDownloads(downloadsRef.current);
            }
        }).catch(console.warn);
        listen("PROGRESS-FINISH", (e) => {
            if (cancelled) return;
            const payload = e.payload as ProgressFinishPayload;
            const current = downloadsRef.current[payload.id];
            if (current && current.progressAction === "Extract") {
                current.msg = payload.msg;
                if (current.progressType === "Indefinite") {
                    current.progress = 1;
                }
                downloadsRef.current = { ...downloadsRef.current, [current.id]: current };
                setDownloads(downloadsRef.current);
            } else if (!payload.success) {
                current.failed = true;
                current.msg = payload.msg;
                downloadsRef.current = { ...downloadsRef.current, [current.id]: current };
                setDownloads(downloadsRef.current);
            }
        }).catch(console.warn);
        return () => {
            cancelled = true;
        };
    }, []);

    return (
        <div className="downloads-popout">
            <header>{getTranslation("DOWNLOADS")}</header>
            {Object.keys(downloads).length === 0 ? (
                <p className="no-downloads">{getTranslation("NO_DOWNLOADS")}</p>
            ) : (
                <>
                    <a
                        href="#"
                        className="clear-downloads"
                        data-tooltip={getTranslation("CLEAR_DOWNLOADS")}
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
