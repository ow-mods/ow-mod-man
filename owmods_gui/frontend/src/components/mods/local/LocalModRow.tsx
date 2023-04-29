import Icon from "@components/common/Icon";
import ModActionButton from "@components/mods/ModActionButton";
import ModHeader from "@components/mods/ModHeader";
import { useGetTranslation } from "@hooks";
import { ModValidationError } from "@types";
import { memo, useCallback } from "react";
import { BsFolderFill, BsGlobe, BsTrashFill } from "react-icons/bs";
import LocalModValidationIcon from "./LocalModValidationIcon";
import { commands } from "@commands";

interface LocalModRowProps {
    uniqueName: string;
    name: string;
    showValidation: boolean;
    errors: ModValidationError[];
    description?: string;
    readme?: boolean;
    subtitle?: string;
    enabled?: boolean;
    onValidationClick?: (p: ModValidationError[]) => void;
    onToggle?: (newState: boolean) => void;
    onOpen?: () => void;
    onUninstall?: () => void;
}

const LocalModRow = memo(function LocalModRow(props: LocalModRowProps) {
    const getTranslation = useGetTranslation();

    const onReadme = useCallback(() => {
        commands.openModReadme({ uniqueName: props.uniqueName }).catch(console.warn);
    }, [props.uniqueName]);

    return (
        <div className="mod-row local">
            <ModHeader name={props.name} subtitle={props.subtitle ?? ""}>
                {props.showValidation && (
                    <LocalModValidationIcon
                        onClickProp={props.onValidationClick}
                        errors={props.errors}
                    />
                )}
                <ModActionButton onClick={props.onOpen} ariaLabel={getTranslation("SHOW_FOLDER")}>
                    <Icon iconType={BsFolderFill} />
                </ModActionButton>
                {props.readme && (
                    <ModActionButton onClick={onReadme} ariaLabel={getTranslation("OPEN_WEBSITE")}>
                        <Icon iconType={BsGlobe} />
                    </ModActionButton>
                )}
                <ModActionButton
                    onClick={props.onUninstall}
                    ariaLabel={getTranslation("UNINSTALL")}
                >
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
            {props.description && <small className="mod-description">{props.description}</small>}
        </div>
    );
});

export default LocalModRow;
