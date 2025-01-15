import { useGetTranslation } from "@hooks";
import ODTooltip from "@components/common/ODTooltip";
import { Box, Chip, Skeleton, TableCell, Theme, Typography, useTheme } from "@mui/material";
import { memo, ReactNode, useMemo } from "react";
import { ExtensionRounded } from "@mui/icons-material";
import ModThumbnail from "./ModThumbnail";

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

const getBgColorFromErrorLevel = (theme: Theme, level?: "warn" | "err") => {
    if (level === "warn") {
        return theme.palette.secondary.dark;
    } else if (level === "err") {
        return theme.palette.error.dark;
    } else {
        return theme.palette.grey[900];
    }
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
    showThumbnail: boolean;
    slug?: string;
    alignActions?: "center" | "start" | "end";
    thumbnailUrl?: string;
    thumbnailClasses?: string;
    description?: string;
    remoteIsLoading?: boolean;
    children?: ReactNode;
    requiresDlc?: boolean;
    isOutdated?: boolean;
    errorLevel?: "warn" | "err";
}

const ModRow = memo(function GenericModRow(props: ModRowProps) {
    const getTranslation = useGetTranslation();
    const theme = useTheme();

    const bgColor = useMemo(
        () => getBgColorFromErrorLevel(theme, props.errorLevel),
        [theme, props.errorLevel]
    );

    const isErr = props.errorLevel === "err";

    const cellStyle = {
        backgroundColor: bgColor,
        paddingTop: theme.spacing(1),
        paddingBottom: theme.spacing(1)
    };

    const formattedDownloads = useMemo(
        () => (props.downloads === -1 ? "—" : formatNumber(props.downloads)),
        [props.downloads]
    );

    const errorList = useMemo(() => {
        if (props.errorLevel) {
            return props.description?.split("\n") ?? [];
        } else {
            return [];
        }
    }, [props.errorLevel, props.description]);

    return (
        <>
            {props.showThumbnail && (
                <TableCell sx={{ paddingRight: 0, ...cellStyle }}>
                    <ModThumbnail
                        isLoading={props.isLoading}
                        remoteIsLoading={props.remoteIsLoading}
                        name={props.name}
                        uniqueName={props.uniqueName}
                        url={props.thumbnailUrl}
                        className={props.thumbnailClasses}
                    />
                </TableCell>
            )}
            <TableCell sx={cellStyle}>
                <Typography display="flex" alignItems="center" variant="subtitle1" noWrap>
                    {props.requiresDlc && (
                        <Box display="inline-block" mr={0.5}>
                            <ODTooltip title={getTranslation("REQUIRES_DLC")}>
                                <Typography>
                                    <ExtensionRounded
                                        sx={{
                                            fontSize: "1.2rem"
                                        }}
                                    />
                                </Typography>
                            </ODTooltip>
                        </Box>
                    )}
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
                                getTranslation("BY", {
                                    author: props.author
                                })
                            )}
                        </Typography>
                    </Box>
                </Typography>
                <Box>
                    <Typography
                        color={isErr ? theme.palette.secondary.light : theme.palette.text.secondary}
                        variant="caption"
                    >
                        {props.isLoading || props.remoteIsLoading ? (
                            <>
                                <Skeleton width={350} />
                                <Skeleton width={275} />
                            </>
                        ) : props.errorLevel ? (
                            <ul>
                                {errorList.map((e) => (
                                    <li key={e}>{e}</li>
                                ))}
                            </ul>
                        ) : (
                            props.description
                        )}
                    </Typography>
                </Box>
            </TableCell>
            <TableCell sx={cellStyle} align="left">
                {props.isLoading || props.remoteIsLoading ? (
                    <Skeleton width={70} />
                ) : (
                    formattedDownloads
                )}
            </TableCell>
            <TableCell sx={cellStyle} align="center">
                <Chip
                    color={isErr ? "default" : props.isOutdated ? "secondary" : "primary"}
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
                    justifyContent={props.alignActions ?? "center"}
                >
                    {props.children}
                </Box>
            </TableCell>
        </>
    );
});

export default ModRow;
