import { Button } from "@mui/material";
import { PlayArrow as PlayIcon } from "@mui/icons-material";
import { useCallback, useEffect, useState } from "react";
import { commands } from "@commands";
import { useGetTranslation } from "@hooks";
import * as dialog from "@tauri-apps/plugin-dialog";
import { simpleOnError } from "../../../errorHandling";
import { listen } from "@events";

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

    useEffect(() => {
        const unsubscribe = listen("protocolInvoke", (protocolPayload) => {
            commands.checkOWML().then((valid) => {
                if (valid && protocolPayload.verb === "runGame") {
                    commands
                        .toggleMod(
                            { uniqueName: protocolPayload.payload, enabled: true, recursive: true },
                            false
                        )
                        .catch(() =>
                            console.warn(`Mod ${protocolPayload.payload} Not Found, Ignoring...`)
                        )
                        .finally(() => {
                            onPlay();
                        });
                }
            });
        });
        commands.popProtocolURL({ id: "run" });
        return unsubscribe;
    }, [onPlay]);

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
