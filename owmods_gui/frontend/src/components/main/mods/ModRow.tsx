import { useGetTranslation } from "@hooks";
import { Box, Paper, Skeleton, TableCell, Typography, Stack, useTheme } from "@mui/material";
import { memo, ReactNode, useMemo } from "react";
import { ExtensionRounded, Download, Person, Label, Warning, Error } from "@mui/icons-material";
import ModThumbnail from "./ModThumbnail";
import ModDetailItem from "./ModDetailItem";
import ModDescription from "./ModDescription";
import RecommendedModBadge from "./RecommendedModBadge";

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
    showThumbnail: boolean;
    slug?: string;
    alignActions?: "center" | "start" | "end";
    thumbnailUrl?: string;
    thumbnailClasses?: string;
    description?: string;
    recommendedBy?: string[];
    remoteIsLoading?: boolean;
    children?: ReactNode;
    requiresDlc?: boolean;
    isOutdated?: boolean;
    errorLevel?: "warn" | "err";
}

const ModRow = memo(function GenericModRow(props: ModRowProps) {
    const getTranslation = useGetTranslation();
    const theme = useTheme();

    const isErr = props.errorLevel === "err";

    const cellStyle = {
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
                <Box
                    sx={{
                        display: "flex",
                        alignItems: "center",
                        justifyContent: "space-between",
                        gap: 1
                    }}
                >
                    <Stack
                        sx={{
                            flex: 1,
                            minWidth: 0,
                            gap: 0.3
                        }}
                    >
                        <Box
                            sx={{
                                display: "flex",
                                alignItems: "center",
                                maxWidth: "100%",
                                whiteSpace: "nowrap",
                                overflow: "hidden",
                                gap: 1
                            }}
                        >
                            {isErr ? (
                                <Error color="error" />
                            ) : (
                                (props.isOutdated || props.errorLevel) && (
                                    <Warning color="warning" />
                                )
                            )}
                            <Typography
                                noWrap
                                sx={{
                                    display: "block",
                                    minWidth: 0,
                                    fontWeight: theme.typography.fontWeightBold
                                }}
                            >
                                {props.isLoading ? <Skeleton width={300} /> : props.name}
                            </Typography>
                            {props.recommendedBy && props.recommendedBy.length > 0 && (
                                <RecommendedModBadge recommendedBy={props.recommendedBy} />
                            )}
                        </Box>
                        <Typography noWrap variant="caption" color="textDisabled">
                            {props.isLoading || props.remoteIsLoading ? (
                                <Skeleton width={250} />
                            ) : (
                                <Box
                                    sx={{
                                        display: "flex",
                                        alignItems: "center",
                                        columnGap: 1
                                    }}
                                >
                                    <ModDetailItem icon={<Label fontSize="inherit" />} truncate>
                                        {props.version}
                                    </ModDetailItem>
                                    <ModDetailItem icon={<Person fontSize="inherit" />} truncate>
                                        {props.author}
                                    </ModDetailItem>
                                    {props.requiresDlc && (
                                        <ModDetailItem
                                            icon={<ExtensionRounded fontSize="inherit" />}
                                        >
                                            {getTranslation("REQUIRES_DLC")}
                                        </ModDetailItem>
                                    )}
                                    <ModDetailItem icon={<Download fontSize="inherit" />}>
                                        {formattedDownloads}
                                    </ModDetailItem>
                                </Box>
                            )}
                        </Typography>
                        {props.errorLevel ? (
                            <Typography variant="caption" sx={{ lineHeight: 1 }}>
                                <Paper
                                    sx={{
                                        bgcolor:
                                            theme.palette[isErr ? "error" : "warning"][
                                                theme.palette.mode
                                            ],
                                        color: theme.palette.getContrastText(
                                            theme.palette[isErr ? "error" : "warning"][
                                                theme.palette.mode
                                            ]
                                        ),
                                        padding: 1,
                                        display: "flex",
                                        flexDirection: "column",
                                        gap: 1
                                    }}
                                >
                                    {errorList.map((e) => (
                                        <Box
                                            sx={{
                                                display: "flex",
                                                alignItems: "center",
                                                gap: 0.5
                                            }}
                                            key={e}
                                        >
                                            <Warning fontSize="inherit" />
                                            {e}
                                        </Box>
                                    ))}
                                </Paper>
                            </Typography>
                        ) : (
                            <ModDescription isLoading={props.isLoading || props.remoteIsLoading}>
                                {props.description}
                            </ModDescription>
                        )}
                    </Stack>
                    <Box
                        sx={{
                            display: "flex",
                            flexDirection: "row",
                            alignContent: "center",
                            justifyContent: props.alignActions ?? "center"
                        }}
                    >
                        {props.children}
                    </Box>
                </Box>
            </TableCell>
        </>
    );
});

export default ModRow;
