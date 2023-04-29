import Icon from "@components/common/Icon";
import { BsArrowDown } from "react-icons/bs";
import DownloadsBadge from "./DownloadsBadge";
import DownloadsPopout from "./DownloadsPopout";
import NavButton from "../nav/NavButton";
import { useGetTranslation } from "@hooks";

const Downloads = () => {
    const getTranslation = useGetTranslation();

    return (
        <li>
            <details role="list">
                <summary>
                    <NavButton labelPlacement="right" ariaLabel={getTranslation("DOWNLOADS")}>
                        <Icon iconType={BsArrowDown} />
                        <DownloadsBadge />
                    </NavButton>
                </summary>
                <DownloadsPopout />
            </details>
        </li>
    );
};

export default Downloads;
