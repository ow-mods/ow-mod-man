import { useGetTranslation } from "@hooks";
import { DoneAllRounded, RemoveDoneRounded } from "@mui/icons-material";
import { Button, ButtonGroup } from "@mui/material";
import { memo } from "react";

export interface LocalModsToggleButtonsProps {
    onToggle: (newVal: boolean) => void;
}

const LocalModsToggleButtons = memo(function LocalModsToolbar(props: LocalModsToggleButtonsProps) {
    const getTranslation = useGetTranslation();

    return (
        <ButtonGroup>
            <Button
                color="neutral"
                startIcon={<DoneAllRounded />}
                onClick={() => props.onToggle(true)}
            >
                {getTranslation("ENABLE_ALL")}
            </Button>
            <Button
                color="neutral"
                startIcon={<RemoveDoneRounded />}
                onClick={() => props.onToggle(false)}
            >
                {getTranslation("DISABLE_ALL")}
            </Button>
        </ButtonGroup>
    );
});

export default LocalModsToggleButtons;
