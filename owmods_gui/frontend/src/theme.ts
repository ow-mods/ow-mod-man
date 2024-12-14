import { PaletteColorOptions, createTheme } from "@mui/material";
import { green, grey, blue, red } from "@mui/material/colors";
import { Theme } from "@types";

declare module "@mui/material/styles" {
    export interface Palette {
        neutral: Palette["primary"];
    }

    export interface PaletteOptions {
        neutral: PaletteOptions["primary"];
    }
}

declare module "@mui/material/Button" {
    export interface ButtonPropsColorOverrides {
        neutral: true;
    }
}

declare module "@mui/material/ButtonGroup" {
    export interface ButtonGroupPropsColorOverrides {
        neutral: true;
    }
}

declare module "@mui/material/CircularProgress" {
    export interface CircularProgressPropsColorOverrides {
        neutral: true;
    }
}

type UserTheme = {
    primary: PaletteColorOptions;
    secondary: PaletteColorOptions;
};

const themeMap: Record<Theme, UserTheme> = {
    Green: {
        primary: {
            main: green[700]
        },
        secondary: {
            main: "#ca7300",
            dark: "#975d2e",
            light: "#ffc380"
        }
    },
    Blue: {
        primary: {
            main: blue[700]
        },
        secondary: {
            main: "#c9724f"
        }
    },
    Red: {
        primary: {
            main: red["700"]
        },
        secondary: {
            main: "#77cfcf"
        }
    },
    Pink: {
        primary: {
            main: "#d15291"
        },
        secondary: {
            main: "#7777cf"
        }
    },
    Purple: {
        primary: {
            main: "#7d3f92"
        },
        secondary: {
            main: "#5b8f82"
        }
    },
    Blurple: {
        primary: {
            main: "#94a0cc"
        },
        secondary: {
            main: "#a67a68"
        }
    },
    GhostlyGreen: {
        primary: {
            main: "#1dc99e"
        },
        secondary: {
            main: "#e4b3f4"
        }
    },
    OuterWildsOrange: {
        primary: {
            main: "#ff7D25"
        },
        secondary: {
            main: "#17c0bb"
        }
    },
    NomaiBlue: {
        primary: {
            main: "#8793ff"
        },
        secondary: {
            main: "#999999"
        }
    },
    NomaiYellow: {
        primary: {
            main: "#fff592"
        },
        secondary: {
            main: "#996638"
        }
    }
};

export const getMuiTheme = (selectedTheme: Theme) => {
    const baseTheme = themeMap[selectedTheme];
    return createTheme({
        palette: {
            mode: "dark",
            ...baseTheme,
            neutral: {
                main: grey[400],
                contrastText: "#1c1c1c"
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
            MuiTab: {
                styleOverrides: {
                    root: {
                        minHeight: 0,
                        padding: 10
                    }
                }
            },
            MuiTabs: {
                styleOverrides: {
                    root: {
                        minHeight: 0,
                        padding: 0
                    }
                }
            },
            MuiTableCell: {
                styleOverrides: {
                    head: {
                        paddingTop: 10,
                        paddingBottom: 10
                    }
                }
            },
            MuiTooltip: {
                styleOverrides: {
                    tooltip: {
                        fontSize: "1em"
                    }
                }
            },
            MuiButton: {
                defaultProps: {
                    variant: "outlined",
                    color: "neutral"
                }
            },
            MuiButtonBase: {
                defaultProps: {
                    color: "neutral"
                }
            },
            MuiButtonGroup: {
                defaultProps: {
                    variant: "outlined",
                    color: "neutral"
                }
            }
        }
    });
};
