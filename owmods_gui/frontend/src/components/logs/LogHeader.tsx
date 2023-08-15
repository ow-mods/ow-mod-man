import { useGetTranslation } from "@hooks";
import { memo } from "react";
import { LogFilter } from "./LogApp";
import { Box, IconButton, Paper, Toolbar, Typography, useTheme } from "@mui/material";
import { DeleteSweepRounded, WarningAmberRounded } from "@mui/icons-material";
import ODTooltip from "@components/common/ODTooltip";
import LogFilters from "./LogFilters";

export interface LogHeaderProps {
    logsLen: number;
    activeFilter: LogFilter;
    setActiveFilter: (filter: LogFilter) => void;
    activeSearch: string;
    setActiveSearch: (newSearch: string) => void;
    isBehind: boolean;
    onClear: () => void;
}

const Counter = memo(function BehindWarning(props: { isBehind: boolean; count: number }) {
    const theme = useTheme();
    const getTranslation = useGetTranslation();

    const Content = (
        <Box display="flex" alignItems="center" justifyContent="flex-end" gap={1}>
            {props.isBehind && <WarningAmberRounded />}
            {getTranslation("LOG_COUNT", { count: props.count.toString() })}
        </Box>
    );

    return (
        <Typography
            textAlign="right"
            flexGrow={1}
            variant="subtitle1"
            color={props.isBehind ? theme.palette.warning.main : undefined}
        >
            {props.isBehind ? (
                <ODTooltip title={getTranslation("LOG_BEHIND")}>{Content}</ODTooltip>
            ) : (
                Content
            )}
        </Typography>
    );
});

const LogHeader = memo(function LogHeader(props: LogHeaderProps) {
    const getTranslation = useGetTranslation();

    return (
        <Paper sx={{ padding: 1 }}>
            <Toolbar disableGutters variant="dense">
                <LogFilters
                    activeFilter={props.activeFilter}
                    activeSearch={props.activeSearch}
                    setActiveFilter={props.setActiveFilter}
                    setActiveSearch={props.setActiveSearch}
                />
                <Counter isBehind={props.isBehind} count={props.logsLen} />
                <Box paddingLeft={2}>
                    <ODTooltip title={getTranslation("CLEAR_LOGS")}>
                        <IconButton onClick={props.onClear}>
                            <DeleteSweepRounded />
                        </IconButton>
                    </ODTooltip>
                </Box>
            </Toolbar>
        </Paper>
    );
});

export default LogHeader;
