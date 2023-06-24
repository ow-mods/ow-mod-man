import { AppBar, Toolbar } from "@mui/material";
import StartGameButton from "./StartGameButton";
import AppIcons from "./AppIcons";
import { memo } from "react";

const TopBar = memo(function TopBar() {
    return (
        <AppBar position="static" component="nav" elevation={2}>
            <Toolbar>
                <AppIcons />
                <div style={{ flexGrow: 1 }} />
                <StartGameButton />
            </Toolbar>
        </AppBar>
    );
});

export default TopBar;
