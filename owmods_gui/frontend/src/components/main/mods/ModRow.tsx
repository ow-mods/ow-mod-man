import { useGetTranslation } from "@hooks";
import { Box, Chip, TableCell, Typography, useTheme } from "@mui/material";
import { ReactNode, memo, useMemo } from "react";

// Stolen from mods website, Rai will never catch me!
const magnitudeMap = [
    { value: 1, symbol: "" },
    { value: 1e3, symbol: "k" },
    { value: 1e6, symbol: "M" },
    { value: 1e9, symbol: "G" },
    { value: 1e12, symbol: "T" },
    { value: 1e15, symbol: "P" },
    { value: 1e18, symbol: "E" }
];

const numberFormatRegex = /\.0+$|(\.[0-9]*[1-9])0+$/;

export const formatNumber = (value: number, digits = 1) => {
    const magnitude = magnitudeMap
        .slice()
        .reverse()
        .find((item) => {
            return value >= item.value;
        });
    return magnitude
        ? (value / magnitude.value).toFixed(digits).replace(numberFormatRegex, "$1") +
              magnitude.symbol
        : "0";
};

export interface OverflowMenuItem {
    icon: ReactNode;
    label: string;
    onClick?: () => void;
}

export interface ModRowProps {
    uniqueName: string;
    name: string;
    author: string;
    downloads: number;
    version: string;
    description?: string;
    children?: ReactNode;
    isOutdated?: boolean;
    errorLevel?: "warn" | "err";
}

const ModRow = memo(function GenericModRow(props: ModRowProps) {
    const getTranslation = useGetTranslation();
    const theme = useTheme();

    const cellStyle = { paddingTop: theme.spacing(1), paddingBottom: theme.spacing(1) };

    const formattedDownloads = useMemo(
        () => (props.downloads === -1 ? "--" : formatNumber(props.downloads)),
        [props.downloads]
    );

    return (
        <>
            <TableCell sx={cellStyle}>
                <Typography variant="subtitle1" noWrap>
                    <Box display="inline-block" mr={1}>
                        {props.name}
                    </Box>
                    <Typography noWrap variant="caption">
                        {getTranslation("BY", { author: props.author })}
                    </Typography>
                </Typography>
                <Box>
                    <Typography variant="caption">{props.description ?? ""}</Typography>
                </Box>
            </TableCell>
            <TableCell sx={cellStyle} align="right">
                {formattedDownloads}
            </TableCell>
            <TableCell sx={cellStyle} align="center">
                <Chip
                    color={props.isOutdated ? "secondary" : "primary"}
                    sx={{
                        width: "100%",
                        minHeight: "100%",
                        padding: theme.spacing(2.5),
                        paddingLeft: 0,
                        paddingRight: 0,
                        "& span": {
                            paddingLeft: 0,
                            paddingRight: 0
                        }
                    }}
                    label={
                        <span>
                            {props.version}
                            <br />
                            {props.isOutdated && getTranslation("OUTDATED")}
                        </span>
                    }
                />
            </TableCell>
            <TableCell sx={cellStyle}>
                <Box display="flex" flexDirection="row" alignContent="center">
                    {props.children}
                </Box>
            </TableCell>
        </>
    );
});

export default ModRow;
