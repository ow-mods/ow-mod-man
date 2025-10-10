import React, { useState, useEffect } from "react";
import { InputAdornment, IconButton, TextField } from "@mui/material";
import { Close as CloseIcon, Search as SearchIcon } from "@mui/icons-material";
import { useDebounce } from "@hooks";

export interface FilterInputProps {
    value: string;
    label: string;
    onChange: (value: string) => void;
    [rest: string | number | symbol]: unknown;
}

const FilterInput: React.FunctionComponent<FilterInputProps> = ({
    value,
    onChange,
    label,
    ...rest
}) => {
    const [filterText, setFilterText] = useState(value);
    const debouncedFilterText = useDebounce(filterText, 200);
    
    useEffect(() => {
        onChange(debouncedFilterText);
    }, [debouncedFilterText, onChange]);

    // Instantly reflect changes on clear, don't debounce
    const onClear = () => {
        setFilterText("");
        onChange("");
    };

    return (
        <TextField
            margin="none"
            size="small"
            onChange={({ currentTarget }) => {
                setFilterText(currentTarget.value);
            }}
            value={filterText}
            placeholder={label}
            variant="outlined"
            {...rest}
            InputProps={{
                startAdornment: (
                    <InputAdornment position="start">
                        <SearchIcon />
                    </InputAdornment>
                ),
                endAdornment: filterText !== "" && (
                    <InputAdornment position="end">
                        <IconButton onClick={onClear} size="small">
                            <CloseIcon fontSize="small" />
                        </IconButton>
                    </InputAdornment>
                )
            }}
        />
    );
};

export default FilterInput;
