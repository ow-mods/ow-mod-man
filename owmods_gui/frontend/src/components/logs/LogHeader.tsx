import { useGetTranslation } from "@hooks";
import { memo } from "react";
import { LogFilter } from "./LogApp";
import {
    Box,
    FormControl,
    IconButton,
    InputLabel,
    MenuItem,
    Paper,
    Select,
    Toolbar,
    Typography,
    useTheme
} from "@mui/material";
import FilterInput from "@components/common/FilterInput";
import { TranslationKey } from "@components/common/TranslationContext";
import { SocketMessageType } from "@types";
import { DeleteSweepRounded } from "@mui/icons-material";
import ODTooltip from "@components/common/ODTooltip";

export interface LogHeaderProps {
    logsLen: number;
    activeFilter: LogFilter;
    setActiveFilter: (filter: LogFilter) => void;
    activeSearch: string;
    setActiveSearch: (newSearch: string) => void;
    onClear: () => void;
}

const LogHeader = memo(function LogHeader({ setActiveSearch, ...props }: LogHeaderProps) {
    const theme = useTheme();
    const getTranslation = useGetTranslation();

    const labelId = "logs-filter-label";

    return (
        <Paper sx={{ padding: 1 }}>
            <Toolbar disableGutters variant="dense">
                <Box maxWidth="30%">
                    <FilterInput
                        value={props.activeSearch}
                        watchValue={false}
                        onChange={(v) => setActiveSearch(v)}
                        label={getTranslation("SEARCH_LOGS")}
                    />
                </Box>
                <Box paddingLeft={theme.spacing(2)} flexGrow={1} maxWidth="30%">
                    <FormControl size="small" fullWidth>
                        <InputLabel id={labelId}>{getTranslation("FILTER")}</InputLabel>
                        <Select
                            labelId={labelId}
                            id="logs-filter-select"
                            value={props.activeFilter}
                            label={getTranslation("FILTER")}
                            onChange={(e) => props.setActiveFilter(e.target.value as LogFilter)}
                        >
                            <MenuItem value="Any">{getTranslation("ANY")}</MenuItem>
                            {Object.keys(SocketMessageType).map((k) => {
                                {
                                    return (
                                        k !== "Fatal" &&
                                        k !== "Quit" && (
                                            <MenuItem key={k} value={k}>
                                                {getTranslation(k as TranslationKey)}
                                            </MenuItem>
                                        )
                                    );
                                }
                            })}
                        </Select>
                    </FormControl>
                </Box>
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
