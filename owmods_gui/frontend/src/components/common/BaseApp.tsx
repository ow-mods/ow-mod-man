import { ThemeProvider } from "@emotion/react";
import { ErrorRounded } from "@mui/icons-material";
import { CssBaseline, Box, CircularProgress, Typography } from "@mui/material";
import theme from "../../theme";
import { TranslationContext } from "./TranslationContext";
import { Language } from "@types";
import { ReactNode, memo } from "react";
import StyledErrorBoundary from "./StyledErrorBoundary";

export interface BaseAppProps {
    isLoading: boolean;
    children: ReactNode;
    language?: Language;
    fatalError?: string;
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
                        {props.fatalError ? (
                            <Typography variant="h5" color="error">
                                <ErrorRounded /> Fatal Error <br /> {props.fatalError?.toString()}
                            </Typography>
                        ) : (
                            <CircularProgress color="neutral" />
                        )}
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
