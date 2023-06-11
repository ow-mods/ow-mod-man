import { useGetTranslation } from "@hooks";
import { Box, Chip, Skeleton, TableCell, Typography, useTheme } from "@mui/material";
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
    isLoading: boolean;
    uniqueName: string;
    name: string;
    author: string;
    downloads: number;
    version: string;
    description?: string;
    remoteIsLoading?: boolean;
    children?: ReactNode;
    isOutdated?: boolean;
    errorLevel?: "warn" | "err";
}

const ModRow = memo(function GenericModRow(props: ModRowProps) {
    const getTranslation = useGetTranslation();
    const theme = useTheme();

    const cellStyle = { paddingTop: theme.spacing(1), paddingBottom: theme.spacing(1) };

    const formattedDownloads = useMemo(
        () => (props.downloads === -1 ? "—" : formatNumber(props.downloads)),
        [props.downloads]
    );

    return (
        <>
            <TableCell sx={cellStyle}>
                <Typography variant="subtitle1" noWrap>
                    <Box display="inline-block" mr={1}>
                        <Typography fontWeight={theme.typography.fontWeightBold}>
                            {props.isLoading ? <Skeleton width={300} /> : props.name}
                        </Typography>
                    </Box>
                    <Box display="inline-block" mr={1}>
                        <Typography noWrap variant="caption" color={theme.palette.text.disabled}>
                            {props.isLoading ? (
                                <></>
                            ) : (
                                getTranslation("BY", { author: props.author })
                            )}
                        </Typography>
                    </Box>
                </Typography>
                <Box>
                    <Typography variant="caption">
                        {props.isLoading || props.remoteIsLoading ? (
                            <>
                                <Skeleton width={350} />
                                <Skeleton width={275} />
                            </>
                        ) : (
                            props.description ?? ""
                        )}
                    </Typography>
                </Box>
            </TableCell>
            <TableCell sx={cellStyle} align="right">
                {props.isLoading || props.remoteIsLoading ? (
                    <Skeleton width={70} />
                ) : (
                    formattedDownloads
                )}
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
            <TableCell sx={cellStyle} align="right">
                <Box
                    display="flex"
                    flexDirection="row"
                    alignContent="center"
                    justifyContent="center"
                >
                    {props.children}
                </Box>
            </TableCell>
        </>
    );
});

export default ModRow;
