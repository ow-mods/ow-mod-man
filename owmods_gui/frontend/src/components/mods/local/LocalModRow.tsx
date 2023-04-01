import Icon from "@components/common/Icon";
import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { useTranslations } from "@hooks";
import { ModValidationError } from "@types";
import { memo } from "react";
import { BsFolderFill, BsTrashFill } from "react-icons/bs";
import LocalModValidationIcon from "./LocalModValidationIcon";

interface LocalModRowProps {
    name: string;
    showValidation: boolean;
    errors: ModValidationError[];
    subtitle?: string;
    enabled?: boolean;
    onValidationClick?: (p: ModValidationError[]) => void;
    onToggle?: (newState: boolean) => void;
    onOpen?: () => void;
    onUninstall?: () => void;
}

const LocalModRow = memo((props: LocalModRowProps) => {
    const [showFolderTooltip, uninstallTooltip] = useTranslations(["SHOW_FOLDER", "UNINSTALL"]);

    return (
        <div className="mod-row local">
            <ModHeader name={props.name} subtitle={props.subtitle ?? ""}>
                {props.showValidation && (
                    <LocalModValidationIcon
                        onClick={props.onValidationClick}
                        errors={props.errors}
                    />
                )}
                <ModActionButton onClick={props.onOpen} ariaLabel={showFolderTooltip}>
                    <Icon iconType={BsFolderFill} />
                </ModActionButton>
                <ModActionButton onClick={props.onUninstall} ariaLabel={uninstallTooltip}>
                    <Icon iconType={BsTrashFill} />
                </ModActionButton>
                <input
                    className="mod-toggle"
                    type="checkbox"
                    aria-label="enabled"
                    role="switch"
                    onChange={(e) => props.onToggle?.(e.target.checked)}
                    checked={props.enabled ?? false}
                    disabled={props.enabled === undefined}
                />
            </ModHeader>
        </div>
    );
});

export default LocalModRow;
