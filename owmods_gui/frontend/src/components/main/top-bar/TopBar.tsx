import { AppBar, Toolbar } from "@mui/material";
import StartGameButton from "./StartGameButton";
import AppIcons from "./AppIcons";

const TopBar = () => {
    return (
        <AppBar position="sticky" component="nav">
            <Toolbar>
                <AppIcons />
                <div style={{ flexGrow: 1 }} />
                <StartGameButton />
            </Toolbar>
        </AppBar>
    );
};

export default TopBar;
