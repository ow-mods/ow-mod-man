import { commands } from "@commands";
import { useGetTranslation } from "@hooks";
import { ModValidationError } from "@types";
import { forwardRef, useImperativeHandle, useMemo, useRef, useState } from "react";
import Modal, { ModalHandle } from "./Modal";

export interface OpenModValidationModalPayload {
    uniqueName: string;
    modName: string;
    errors: ModValidationError[];
}

export interface ModValidationModalHandle {
    open: (payload: OpenModValidationModalPayload) => void;
    close: () => void;
}

const ValidationError = (props: ModValidationError) => {
    const getTranslation = useGetTranslation();
    const message = getTranslation(props.errorType, { payload: props.payload ?? "" });
    return <li>{message}</li>;
};

const ModValidationModal = forwardRef(function ModValidationModal(_: object, ref) {
    const modalRef = useRef<ModalHandle>();
    const [uniqueName, setUniqueName] = useState<string>("");
    const [modName, setModName] = useState<string>("");
    const [errors, setErrors] = useState<ModValidationError[]>([]);
    const getTranslation = useGetTranslation();

    useImperativeHandle(
        ref,
        () => ({
            open: (payload: OpenModValidationModalPayload) => {
                setModName(payload.modName);
                setUniqueName(payload.uniqueName);
                setErrors(payload.errors);
                modalRef.current?.open();
            },
            close: () => {
                modalRef.current?.close();
            }
        }),
        []
    );

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
            heading={getTranslation("VALIDATION_HEADER", { name: modName })}
            confirmText={canFix ? getTranslation("FIX") : getTranslation("OK")}
            showCancel={canFix}
            cancelText={getTranslation("DONT_FIX")}
            onConfirm={onConfirm}
            ref={modalRef}
        >
            <h6>{getTranslation("VALIDATION_MESSAGE", { name: modName })}</h6>
            <ul>
                {errors.map((e) => (
                    <ValidationError key={e.errorType + e.payload} {...e} />
                ))}
            </ul>
            {canFix && <p>{getTranslation("VALIDATION_FIX_MESSAGE")}</p>}
        </Modal>
    );
});

export default ModValidationModal;
