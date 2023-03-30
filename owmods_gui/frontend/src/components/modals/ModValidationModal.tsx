import { commands } from "@commands";
import { useTranslation, useTranslations } from "@hooks";
import { ModValidationError } from "@types";
import { MutableRefObject, useEffect, useMemo, useRef, useState } from "react";
import Modal from "./Modal";

export interface OpenModValidationModalPayload {
    uniqueName: string;
    modName: string;
    errors: ModValidationError[];
}

export interface ModValidationModalProps {
    open: MutableRefObject<(payload: OpenModValidationModalPayload) => void>;
}

const ValidationError = (props: ModValidationError) => {
    const message = useTranslation(props.errorType, { payload: props.payload ?? "" });
    return <li>{message}</li>;
};

const ModValidationModal = (props: ModValidationModalProps) => {
    const openInternal = useRef<() => void>(() => null);
    const [uniqueName, setUniqueName] = useState<string>("");
    const [modName, setModName] = useState<string>("");
    const [errors, setErrors] = useState<ModValidationError[]>([]);

    const [fix, ok, dontFix, fixMessage] = useTranslations([
        "FIX",
        "OK",
        "DONT_FIX",
        "VALIDATION_FIX_MESSAGE"
    ]);

    const header = useTranslation("VALIDATION_HEADER", { name: modName });
    const message = useTranslation("VALIDATION_MESSAGE", { name: modName });

    useEffect(() => {
        props.open.current = (payload: OpenModValidationModalPayload) => {
            setModName(payload.modName);
            setUniqueName(payload.uniqueName);
            setErrors(payload.errors);
            openInternal.current();
        };
    }, [openInternal.current]);

    const onConfirm = () => {
        if (canFix) {
            commands
                .fixDeps({ uniqueName })
                .then(() => commands.refreshLocalDb())
                .catch(console.warn);
        }
    };

    const canFix = useMemo(() => {
        return errors.every((e) => e.errorType === "DisabledDep" || e.errorType === "MissingDep");
    }, [errors]);

    return (
        <Modal
            heading={header}
            confirmText={canFix ? fix : ok}
            showCancel={canFix}
            cancelText={dontFix}
            onConfirm={onConfirm}
            open={openInternal}
        >
            <h6>{message}</h6>
            <ul>
                {errors.map((e) => (
                    <ValidationError key={e.errorType + e.payload} {...e} />
                ))}
            </ul>
            {canFix && <p>{fixMessage}</p>}
        </Modal>
    );
};

export default ModValidationModal;
