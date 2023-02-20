import { listen } from "@tauri-apps/api/event";

type LogType = "DEBUG" | "INFO" | "WARNING" | "ERROR";

interface LogMessage {
    log_type: LogType;
    target?: string;
    message: string;
}

export const startLogListen = () => {
    listen("LOG", (e) => {
        const msg = e.payload as LogMessage;
        if (msg.target !== "progress" && !msg.target?.startsWith("game")) {
            switch (msg.log_type) {
                case "DEBUG":
                    console.debug(msg.message);
                    break;
                case "INFO":
                    console.info(msg.message);
                    break;
                case "WARNING":
                    console.warn(msg.message);
                    break;
                case "ERROR":
                    console.error(msg.message);
                    break;
                default:
                    console.log(msg.message);
                    break;
            }
        }
    });
};
