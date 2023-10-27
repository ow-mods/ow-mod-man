import { commands } from "@commands";
import { ErrorInfo } from "react";

export const simpleOnError = (err: string) => {
    commands.logError({
        err: err.toString()
    });
};

export const onError = (err: Error, info: ErrorInfo) => {
    commands.logError({
        err: `${err?.toString() ?? "null"}\nAt: ${info.componentStack}`
    });
};

export const basicFallbackRender = ({ error }: { error: unknown }) => (
    <p
        style={{ color: "rgb(255, 83, 83)", margin: "50vh 10vw", textAlign: "center" }}
    >{`Fatal Error: ${error}`}</p>
);
