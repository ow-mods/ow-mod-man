import { ThemeProvider } from "@emotion/react";
import { CssBaseline, Box, CircularProgress } from "@mui/material";
import theme from "../../theme";
import { TranslationContext } from "./TranslationContext";
import { Language } from "@types";
import { ReactNode, memo } from "react";
import StyledErrorBoundary from "./StyledErrorBoundary";

export interface BaseAppProps {
    isLoading: boolean;
    children: ReactNode;
    language?: Language;
}

const BaseApp = memo(function BaseApp(props: BaseAppProps) {
    return (
        <ThemeProvider theme={theme}>
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
