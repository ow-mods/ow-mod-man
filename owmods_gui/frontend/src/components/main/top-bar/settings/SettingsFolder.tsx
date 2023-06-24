import { OpenFileInput } from "@components/common/FileInput";
import ODTooltip from "@components/common/ODTooltip";
import { useGetTranslation } from "@hooks";
import { SettingsTextProps } from "./SettingsText";
import { Box } from "@mui/material";

const SettingsFolder = (props: SettingsTextProps) => {
    const getTranslation = useGetTranslation();

    const onChange = (e: string) => {
        props.onChange?.(props.id, e);
    };

    return (
        <ODTooltip title={props.tooltip}>
            <Box>
                <OpenFileInput
                    id={props.id}
                    label={props.label}
                    value={props.value}
                    onChange={onChange}
                    dialogOptions={{
                        defaultPath: props.value,
                        directory: true,
                        multiple: false,
                        title: getTranslation("SELECT", { name: props.label })
                    }}
                />
            </Box>
        </ODTooltip>
    );
};

export default SettingsFolder;
