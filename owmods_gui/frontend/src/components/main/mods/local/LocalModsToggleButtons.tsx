import { useGetTranslation } from "@hooks";
import { Button, ButtonGroup } from "@mui/material";
import { memo } from "react";

export interface LocalModsToggleButtonsProps {
    onToggle: (newVal: boolean) => void;
}

const LocalModsToggleButtons = memo(function LocalModsToolbar(props: LocalModsToggleButtonsProps) {
    const getTranslation = useGetTranslation();

    return (
        <ButtonGroup>
            <Button onClick={() => props.onToggle(true)}>{getTranslation("ENABLE_ALL")}</Button>
            <Button onClick={() => props.onToggle(false)}>{getTranslation("DISABLE_ALL")}</Button>
        </ButtonGroup>
    );
});

export default LocalModsToggleButtons;
