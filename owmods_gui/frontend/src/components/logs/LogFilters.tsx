import FilterInput from "@components/common/FilterInput";
import { TranslationKey } from "@components/common/TranslationContext";
import { useGetTranslation } from "@hooks";
import { Box, FormControl, InputLabel, Select, MenuItem, useTheme } from "@mui/material";
import { SocketMessageType } from "@types";
import { memo } from "react";
import { LogFilter } from "./LogApp";

export interface LogFilterProps {
    activeFilter: LogFilter;
    setActiveFilter: (filter: LogFilter) => void;
    activeSearch: string;
    setActiveSearch: (newSearch: string) => void;
}

const LogFilters = memo(function LogFilter(props: LogFilterProps) {
    const theme = useTheme();
    const getTranslation = useGetTranslation();

    const labelId = "logs-filter-label";

    return (
        <>
            <Box maxWidth="30%">
                <FilterInput
                    value={props.activeSearch}
                    watchValue={false}
                    onChange={(v) => props.setActiveSearch(v)}
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
        </>
    );
});

export default LogFilters;
