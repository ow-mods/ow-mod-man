import Icon from "@components/common/Icon";
import { useTranslations } from "@hooks";
import { ModValidationError } from "@types";
import { useCallback } from "react";
import { BsExclamationDiamondFill } from "react-icons/bs";
import ModActionButton from "../ModActionButton";

export interface LocalModValidationIconProps {
    errors: ModValidationError[];
    onClick?: (errors: ModValidationError[]) => void;
}

const LocalModValidationIcon = (props: LocalModValidationIconProps) => {
    const [hasErrors, hasWarning] = useTranslations(["MOD_HAS_ERRORS", "MOD_HAS_WARNINGS"]);

    const onClick = useCallback(() => {
        props.onClick?.(props.errors);
    }, [props.errors]);

    if (props.errors.length === 0) {
        return <></>;
    } else {
        const errorInList =
            props.errors.find(
                (e) =>
                    e.errorType === "MissingDep" ||
                    e.errorType === "DisabledDep" ||
                    e.errorType === "InvalidManifest" ||
                    e.errorType === "DuplicateMod"
            ) !== undefined;

        return (
            <ModActionButton
                onClick={onClick}
                className={errorInList ? "mod-error" : "mod-warning"}
                ariaLabel={errorInList ? hasErrors : hasWarning}
            >
                <Icon iconType={BsExclamationDiamondFill} />
            </ModActionButton>
        );
    }
};

export default LocalModValidationIcon;
