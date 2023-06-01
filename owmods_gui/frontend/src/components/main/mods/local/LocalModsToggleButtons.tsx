import { useGetTranslation } from "@hooks";
import { Button, ButtonGroup, useTheme } from "@mui/material";

export interface LocalModsToggleButtonsProps {
    onToggle: (newVal: boolean) => void;
}

const LocalModsToggleButtons = (props: LocalModsToggleButtonsProps) => {
    const getTranslation = useGetTranslation();
    const theme = useTheme();

    const buttonStyle = {
        padding: theme.spacing(1.5)
    };

    return (
        <ButtonGroup>
            <Button style={buttonStyle} onClick={() => props.onToggle(true)}>
                {getTranslation("ENABLE_ALL")}
            </Button>
            <Button style={buttonStyle} onClick={() => props.onToggle(false)}>
                {getTranslation("DISABLE_ALL")}
            </Button>
        </ButtonGroup>
    );
};

export default LocalModsToggleButtons;
