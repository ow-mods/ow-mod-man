import { AppBar, Toolbar } from "@mui/material";
import StartGameButton from "./StartGameButton";
import AppIcons from "./AppIcons";
import { memo } from "react";

const TopBar = memo(function TopBar() {
    return (
        <AppBar position="sticky" component="nav">
            <Toolbar>
                <AppIcons />
                <div style={{ flexGrow: 1 }} />
                <StartGameButton />
            </Toolbar>
        </AppBar>
    );
});

export default TopBar;
