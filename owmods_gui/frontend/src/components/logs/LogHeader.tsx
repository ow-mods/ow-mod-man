import { useGetTranslation } from "@hooks";
import { memo } from "react";
import { LogFilter } from "./LogApp";
import { Box, IconButton, Paper, Toolbar, Typography, useTheme } from "@mui/material";
import { DeleteSweepRounded } from "@mui/icons-material";
import ODTooltip from "@components/common/ODTooltip";
import LogFilters from "./LogFilters";

export interface LogHeaderProps {
    logsLen: number;
    activeFilter: LogFilter;
    setActiveFilter: (filter: LogFilter) => void;
    activeSearch: string;
    setActiveSearch: (newSearch: string) => void;
    onClear: () => void;
}

const LogHeader = memo(function LogHeader(props: LogHeaderProps) {
    const theme = useTheme();
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
                <Typography textAlign="right" flexGrow={1} variant="subtitle1">
                    {getTranslation("LOG_COUNT", { count: props.logsLen.toString() })}
                </Typography>
                <Box paddingLeft={theme.spacing(2)}>
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
