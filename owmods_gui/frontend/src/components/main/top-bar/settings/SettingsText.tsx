import ODTooltip from "@components/common/ODTooltip";
import { TextField } from "@mui/material";
import { SettingsRowProps } from "./SettingsForm";

export interface SettingsTextProps extends SettingsRowProps {
    value: string;
    onChange?: (id: string, newVal: string) => void;
}

const SettingsText = (props: SettingsTextProps) => {
    return (
        <ODTooltip title={props.tooltip}>
            <TextField
                id={props.id}
                label={props.label}
                value={props.value}
                variant="outlined"
                fullWidth
                onChange={(e) => props.onChange?.(props.id, e.target.value)}
            />
        </ODTooltip>
    );
};

export default SettingsText;
