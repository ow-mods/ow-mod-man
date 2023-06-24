import { CheckCircleRounded, ErrorRounded } from "@mui/icons-material";
import { Box, Card, CardContent, LinearProgress, Typography, useTheme } from "@mui/material";
import { ProgressBar } from "@types";
import { memo } from "react";
import { determineProgressVariant } from "./DownloadsIcon";

const DownloadRow = memo(function DownloadRow(props: ProgressBar) {
    const theme = useTheme();
    const done = props.success !== undefined && props.success !== null;

    const percent = (props.progress / props.len) * 100;

    return (
        <Card>
            <CardContent>
                <Typography marginBottom={theme.spacing(1)}>{props.message}</Typography>
                <Box display="flex" alignItems="center" gap={theme.spacing(1)}>
                    {done ? (
                        props.success ? (
                            <CheckCircleRounded fontSize="small" color="primary" />
                        ) : (
                            <ErrorRounded fontSize="small" color="error" />
                        )
                    ) : (
                        <Typography variant="caption" whiteSpace="nowrap">
                            {props.progressType === "Definite" ? `${Math.round(percent)}%` : "—%"}
                        </Typography>
                    )}

                    <Box width="100%">
                        <LinearProgress
                            variant={determineProgressVariant(props)}
                            value={percent}
                            color={done ? (props.success ? "primary" : "error") : "secondary"}
                        />
                    </Box>
                </Box>
            </CardContent>
        </Card>
    );
});

export default DownloadRow;
