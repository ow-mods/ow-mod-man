import { commands, hooks } from "@commands";
import Icon from "@components/common/Icon";
import { useGetTranslation } from "@hooks";
import { ProgressBar } from "@types";
import { BsCheck2, BsTrashFill, BsXCircleFill } from "react-icons/bs";

const ActiveDownload = (props: ProgressBar) => {
    const done = props.success !== null;
    const doneClass = props.success ? "download-done" : "download-failed";

    return (
        <div className={`downloads-row ${done ? doneClass : ""}`}>
            <p className="download-header fix-icons">
                <Icon iconClassName="download-icon-failure" iconType={BsXCircleFill} />
                <Icon iconClassName="download-icon-success" iconType={BsCheck2} /> {props.message}
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
    const getTranslation = useGetTranslation();

    const downloads = hooks.getDownloads("PROGRESS-UPDATE")[1] ?? {};

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
                        onClick={() => commands.clearDownloads()}
                    >
                        <Icon iconType={BsTrashFill} />
                    </a>
                    {Object.values(downloads).map((d) => (
                        <ActiveDownload key={d.id} {...d} />
                    ))}
                </>
            )}
        </div>
    );
};

export default DownloadsPopout;
