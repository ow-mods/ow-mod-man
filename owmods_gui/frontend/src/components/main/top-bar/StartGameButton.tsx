import { Button } from "@mui/material";
import { PlayArrow as PlayIcon } from "@mui/icons-material";
import { useCallback, useState } from "react";
import { commands } from "@commands";
import { useGetTranslation } from "@hooks";
import { dialog } from "@tauri-apps/api";
import { simpleOnError } from "@components/common/StyledErrorBoundary";

const StartGameButton = () => {
    const getTranslation = useGetTranslation();
    const [areLogsStarting, setLogsStarting] = useState(false);

    const onPlay = useCallback(() => {
        const start = () =>
            commands
                .startLogs()
                .catch(simpleOnError)
                .finally(() => setLogsStarting(false));
        setLogsStarting(true);
        const task = async () => {
            const hasIssues = await commands.checkDBForIssues();
            const skipWarning = (await commands.getGuiConfig()).noWarning;
            if (!skipWarning && hasIssues) {
                const yes = await dialog.ask(getTranslation("LAUNCH_ANYWAY"), {
                    type: "warning",
                    title: getTranslation("CONFIRM")
                });
                if (yes) {
                    start();
                } else {
                    setLogsStarting(false);
                }
            } else {
                start();
            }
        };
        task();
    }, [getTranslation]);

    return (
        <span>
            <Button
                onClick={onPlay}
                size="large"
                variant="contained"
                color="primary"
                disabled={areLogsStarting}
                startIcon={<PlayIcon />}
            >
                {getTranslation("RUN_GAME")}
            </Button>
        </span>
    );
};

export default StartGameButton;
