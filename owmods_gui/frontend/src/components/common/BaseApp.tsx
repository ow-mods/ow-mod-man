import { ThemeProvider } from "@emotion/react";
import { ErrorRounded } from "@mui/icons-material";
import { CssBaseline, Box, CircularProgress, Typography } from "@mui/material";
import theme from "../../theme";
import { TranslationContext } from "./TranslationContext";
import { Language } from "@types";
import { ReactNode } from "react";

export interface BaseAppProps {
    isLoading: boolean;
    children: ReactNode;
    language?: Language;
    fatalError?: string;
}

const BaseApp = (props: BaseAppProps) => {
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
                        {props.isLoading ? (
                            <CircularProgress color="neutral" />
                        ) : (
                            <Typography variant="h5" color="error">
                                <ErrorRounded /> Fatal Error <br /> {props.fatalError}
                            </Typography>
                        )}
                    </Box>
                ) : (
                    <TranslationContext.Provider value={props.language!}>
                        <Box display="flex" flexDirection="column" height="100%">
                            {props.children}
                        </Box>
                    </TranslationContext.Provider>
                )}
            </CssBaseline>
        </ThemeProvider>
    );
};

export default BaseApp;
