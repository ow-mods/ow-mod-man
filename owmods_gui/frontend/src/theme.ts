import { createTheme } from "@mui/material";
import { green, grey, red } from "@mui/material/colors";

export default createTheme({
    palette: {
        mode: "dark",
        primary: {
            main: green[700]
        },
        secondary: {
            main: "#ca7300",
            dark: "#975d2e",
            light: "#ffc380"
        },
        error: {
            main: red[500],
            dark: "#7e1e1e"
        }
    },
    components: {
        MuiCssBaseline: {
            styleOverrides: {
                body: {
                    innerHeight: "100vh",
                    overflowY: "hidden"
                },
                "*::-webkit-scrollbar": {
                    width: "1em",
                    cursor: "pointer"
                },
                "*::-webkit-scrollbar-track": {
                    background: grey[800],
                    borderRadius: "5px"
                },
                "*::-webkit-scrollbar-thumb": {
                    background: grey[600],
                    border: `2px solid ${grey[800]}`,
                    borderRadius: "5px",
                    "&:hover": {
                        background: grey[600]
                    }
                }
            }
        },
        MuiTooltip: {
            styleOverrides: {
                tooltip: {
                    fontSize: "1em"
                }
            }
        }
    }
});
