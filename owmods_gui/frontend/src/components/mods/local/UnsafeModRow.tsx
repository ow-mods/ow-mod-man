import { hooks } from "@commands";
import CenteredSpinner from "@components/common/CenteredSpinner";
import { memo } from "react";
import ValidModRow from "./ValidModRow";
import { OpenModValidationModalPayload } from "@components/modals/ModValidationModal";
import FailedModRow from "./FailedModRow";

export interface UnsafeModRowProps {
    uniqueName: string;
    onValidationClicked?: (p: OpenModValidationModalPayload) => void;
}

const UnsafeModRow = memo((props: UnsafeModRowProps) => {
    const [status, mod, err] = hooks.getLocalMod("LOCAL-REFRESH", { uniqueName: props.uniqueName });

    if (status === "Loading" && mod === null) {
        return <CenteredSpinner />;
    } else if (status === "Error") {
        return <p className="mod-row center">{err!.toString()}</p>;
    } else {
        if (mod === null) {
            return <></>;
        } else if (mod!.loadState === "invalid") {
            return <FailedModRow mod={mod!.mod} onValidationClick={props.onValidationClicked} />;
        } else {
            return <ValidModRow mod={mod!.mod} onValidationClick={props.onValidationClicked} />;
        }
    }
});

export default UnsafeModRow;
