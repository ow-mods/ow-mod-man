import { commands } from "@commands";

export const simpleOnError = (err: string) => {
    commands.logError({
        err: err.toString()
    });
};

export const onError = (err: unknown, info: { componentStack: string }) => {
    commands.logError({
        err: `${err?.toString() ?? "null"}\nAt: ${info.componentStack}`
    });
};

export const basicFallbackRender = ({ error }: { error: unknown }) => (
    <p
        style={{ color: "rgb(255, 83, 83)", margin: "50vh 10vw", textAlign: "center" }}
    >{`Fatal Error: ${error}`}</p>
);
