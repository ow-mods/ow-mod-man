import Icon from "@components/Icon";
import { BsArrowDown } from "react-icons/bs";
import DownloadsBadge from "./DownloadsBadge";
import DownloadsPopout from "./DownloadsPopout";
import NavButton from "../nav/NavButton";
import { useTranslation } from "@hooks";

const Downloads = () => {
    const downloads = useTranslation("DOWNLOADS");

    return (
        <li>
            <details role="list">
                <summary>
                    <NavButton labelPlacement="right" ariaLabel={downloads}>
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
