import { useGetTranslation } from "@hooks";
import {
    Box,
    Chip,
    ListItemIcon,
    Menu,
    MenuItem,
    TableCell,
    TableRow,
    Typography,
    useTheme
} from "@mui/material";
import { ReactNode, useState, MouseEvent } from "react";
import ModActionIcon from "./ModActionIcon";
import { MoreVertRounded } from "@mui/icons-material";

export interface OverflowMenuItem {
    icon: ReactNode;
    label: string;
    onClick?: () => void;
}

export interface ModRowProps {
    uniqueName: string;
    name: string;
    author: string;
    downloads: string;
    version: string;
    description?: string;
    children?: ReactNode;
    overflow?: OverflowMenuItem[];
    isOutdated?: boolean;
    errorLevel?: "warn" | "err";
}

const ModRow = (props: ModRowProps) => {
    const getTranslation = useGetTranslation();
    const theme = useTheme();
    const [anchorEl, setAnchorEl] = useState<HTMLElement | null>(null);
    const open = Boolean(anchorEl);
    const onClick = (event: MouseEvent<HTMLButtonElement>) => {
        setAnchorEl(event.currentTarget);
    };
    const onClose = () => {
        setAnchorEl(null);
    };

    const cellStyle = { paddingTop: theme.spacing(1), paddingBottom: theme.spacing(1) };

    const overflowId = `${props.uniqueName}-actions-overflow`;
    const overflowButtonId = `${props.uniqueName}-actions-overflow-button`;

    return (
        <TableRow key={props.uniqueName}>
            <TableCell sx={cellStyle}>
                <Typography variant="subtitle1">
                    <Box display="inline-block" mr={2}>
                        {props.name}
                    </Box>
                    <Typography variant="caption">
                        {getTranslation("BY", { author: props.author })}
                    </Typography>
                    <Typography variant="caption" />
                </Typography>
                <Box>
                    <Typography variant="caption">{props.description ?? ""}</Typography>
                </Box>
            </TableCell>
            <TableCell sx={cellStyle} align="right">
                {props.downloads}
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
                    {props.overflow && (
                        <>
                            <ModActionIcon
                                onClick={onClick}
                                id={overflowButtonId}
                                aria-controls={open ? overflowId : undefined}
                                aria-haspopup="true"
                                aria-expanded={open ? "true" : undefined}
                                label={getTranslation("MORE")}
                                icon={<MoreVertRounded />}
                            />
                            <Menu
                                id={overflowId}
                                anchorEl={anchorEl}
                                open={open}
                                onClose={onClose}
                                MenuListProps={{
                                    "aria-labelledby": overflowButtonId
                                }}
                            >
                                {props.overflow.map((o) => (
                                    <MenuItem key={o.label} onClick={o.onClick}>
                                        <ListItemIcon>{o.icon}</ListItemIcon>
                                        {o.label}
                                    </MenuItem>
                                ))}
                            </Menu>
                        </>
                    )}
                </Box>
            </TableCell>
        </TableRow>
    );
};

export default ModRow;
