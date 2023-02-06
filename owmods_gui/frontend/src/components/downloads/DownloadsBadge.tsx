export interface DownloadsBadgeProps {
    count: number;
}

const DownloadsBadge = (props: DownloadsBadgeProps) => {
    return (
        <div className={`download-badge${props.count === 0 ? " d-none" : ""}`}>{props.count}</div>
    );
};

export default DownloadsBadge;
