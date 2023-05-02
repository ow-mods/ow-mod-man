import Icon from "@components/common/Icon";
import { useGetTranslation } from "@hooks";
import { ModValidationError } from "@types";
import { useCallback } from "react";
import { BsExclamationDiamondFill } from "react-icons/bs";
import ModActionButton from "../ModActionButton";

export interface LocalModValidationIconProps {
    errors: ModValidationError[];
    onClickProp?: (errors: ModValidationError[]) => void;
}

const LocalModValidationIcon = ({ errors, onClickProp }: LocalModValidationIconProps) => {
    const getTranslation = useGetTranslation();

    const onClick = useCallback(() => {
        onClickProp?.(errors);
    }, [errors, onClickProp]);

    if (errors.length === 0) {
        return <></>;
    } else {
        const errorInList =
            errors.find(
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
                ariaLabel={
                    errorInList
                        ? getTranslation("MOD_HAS_ERRORS")
                        : getTranslation("MOD_HAS_WARNINGS")
                }
            >
                <Icon iconType={BsExclamationDiamondFill} />
            </ModActionButton>
        );
    }
};

export default LocalModValidationIcon;
