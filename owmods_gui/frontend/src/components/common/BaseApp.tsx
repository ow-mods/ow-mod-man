import { ThemeProvider } from "@emotion/react";
import { CssBaseline, Box, CircularProgress } from "@mui/material";
import { TranslationContext } from "./TranslationContext";
import { Language, Theme } from "@types";
import { ReactNode, memo } from "react";
import StyledErrorBoundary from "./StyledErrorBoundary";
import { getMuiTheme } from "../../theme";

export interface BaseAppProps {
    isLoading: boolean;
    children: ReactNode;
    language?: Language;
    theme?: Theme;
}

const BaseApp = memo(function BaseApp(props: BaseAppProps) {
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
