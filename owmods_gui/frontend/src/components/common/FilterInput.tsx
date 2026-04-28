import React, { memo, useRef, useState } from "react";
import { InputAdornment, IconButton, TextField } from "@mui/material";
import { Close as CloseIcon, Search as SearchIcon } from "@mui/icons-material";

export interface FilterInputProps {
    value: string;
    label: string;
    onChange: (value: string) => void;
    [rest: string | number | symbol]: unknown;
}

const FilterInput: React.FunctionComponent<FilterInputProps> = memo(function FilterInput({
    value,
    onChange,
    label,
    ...rest
}: FilterInputProps) {
    const [tempFilter, setTemp] = useState<string | null>(null);
    const currTimeout = useRef<number | null>(null);

    const setText = (newText: string) => {
        setTemp(newText);
        if (currTimeout.current) {
            clearTimeout(currTimeout.current);
            currTimeout.current = null;
        }
        currTimeout.current = setTimeout(() => {
            onChange(newText);
            setTemp(null);
        }, 200);
    };

    return (
        <TextField
            margin="none"
            size="small"
            onChange={({ currentTarget }) => {
                setText(currentTarget.value);
            }}
            value={tempFilter ?? value}
            placeholder={label}
            variant="outlined"
            {...rest}
            slotProps={{
                input: {
                    startAdornment: (
                        <InputAdornment position="start">
                            <SearchIcon />
                        </InputAdornment>
                    ),
                    endAdornment: value !== "" && (
                        <InputAdornment position="end">
                            <IconButton
                                onClick={() => {
                                    onChange("");
                                    if (currTimeout.current) {
                                        clearTimeout(currTimeout.current);
                                    }
                                    setTemp(null);
                                }}
                                size="small"
                            >
                                <CloseIcon fontSize="small" />
                            </IconButton>
                        </InputAdornment>
                    )
                }
            }}
        />
    );
});

export default FilterInput;
