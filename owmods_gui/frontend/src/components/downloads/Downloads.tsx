import Icon from "@components/Icon";
import { FaArrowDown } from "react-icons/fa";
import DownloadsBadge from "./DownloadsBadge";
import DownloadsPopout from "./DownloadsPopout";
import NavButton from "../nav/NavButton";

const Downloads = () => {
    return (
        <li>
            <details role="list">
                <summary>
                    <NavButton labelPlacement="right" ariaLabel="Downloads">
                        <Icon iconType={FaArrowDown} />
                        <DownloadsBadge count={3} />
                    </NavButton>
                </summary>
                <DownloadsPopout />
            </details>
        </li>
    );
};

export default Downloads;
