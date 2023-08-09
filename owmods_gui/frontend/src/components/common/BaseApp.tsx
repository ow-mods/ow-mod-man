import { ThemeProvider } from "@emotion/react";
import { CssBaseline, Box, CircularProgress } from "@mui/material";
import { TranslationContext } from "./TranslationContext";
import { Language, Theme } from "@types";
import { ReactNode, memo, useEffect } from "react";
import StyledErrorBoundary from "./StyledErrorBoundary";
import { getMuiTheme } from "../../theme";
import rainbowCss from "../../rainbow.css?raw";

export interface BaseAppProps {
    isLoading: boolean;
    children: ReactNode;
    language?: Language;
    theme?: Theme;
    usesRainbow?: boolean;
}

const BaseApp = memo(function BaseApp(props: BaseAppProps) {
    useEffect(() => {
        const rainbowElem = document.getElementById("rainbow-style");
        if (rainbowElem) {
            if (props.usesRainbow) {
                rainbowElem.innerHTML = rainbowCss;
            } else {
                rainbowElem.innerHTML = "";
            }
        }
    }, [props.usesRainbow]);

    return (
        <ThemeProvider theme={getMuiTheme(props.theme ?? Theme.Green)}>
            <CssBaseline>
                {props.isLoading ? (
                    <Box
                        width="100%"
                        height="100%"
                        display="flex"
                        alignItems="center"
                        justifyContent="center"
                    >
                        <CircularProgress color="neutral" />
                    </Box>
                ) : (
                    <TranslationContext.Provider value={props.language!}>
                        <StyledErrorBoundary center>
                            <Box display="flex" flexDirection="column" height="100%">
                                {props.children}
                            </Box>
                        </StyledErrorBoundary>
                    </TranslationContext.Provider>
                )}
            </CssBaseline>
        </ThemeProvider>
    );
});

export default BaseApp;
