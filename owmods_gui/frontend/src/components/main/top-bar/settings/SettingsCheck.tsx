import ODTooltip from "@components/common/ODTooltip";
import { FormControlLabel, Checkbox } from "@mui/material";
import { SettingsRowProps } from "./SettingsForm";

export interface SettingsCheckProps extends SettingsRowProps {
    value: boolean;
    onChange?: (id: string, newVal: boolean) => void;
}

const SettingsCheck = (props: SettingsCheckProps) => {
    return (
        <ODTooltip title={props.tooltip}>
            <FormControlLabel
                onChange={(_, newVal) => props.onChange?.(props.id, newVal)}
                checked={props.value}
                control={<Checkbox />}
                label={props.label}
            />
        </ODTooltip>
    );
};

export default SettingsCheck;
