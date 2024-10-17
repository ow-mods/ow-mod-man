import { TranslationKey } from "@components/common/TranslationContext";
import { useGetTranslation } from "@hooks";
import { FormControl, InputLabel, Select, MenuItem } from "@mui/material";
import { SettingsRowProps } from "./SettingsForm";

export interface SettingsSelectProps extends SettingsRowProps {
    value: string;
    options: readonly string[];
    translate: boolean;
    onChange?: (id: string, newVal: string) => void;
    nameMap?: Record<string, string>;
}

const SettingsSelect = (props: SettingsSelectProps) => {
    const getTranslation = useGetTranslation();

    const labelId = `${props.id}-label`;

    return (
        <FormControl fullWidth>
            <InputLabel id={labelId}>{props.label}</InputLabel>
            <Select
                labelId={labelId}
                id={props.id}
                value={props.value}
                label={props.label}
                onChange={(e) => props.onChange?.(props.id, e.target.value)}
            >
                {props.options.map((o) => (
                    <MenuItem key={o} value={o}>
                        {props.translate
                            ? getTranslation(o as TranslationKey)
                            : (props.nameMap?.[o] ?? o)}
                    </MenuItem>
                ))}
            </Select>
        </FormControl>
    );
};

export default SettingsSelect;
