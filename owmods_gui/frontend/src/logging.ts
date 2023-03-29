import { listen } from "@tauri-apps/api/event";
import { LogPayload } from "@types";

type LogType = "DEBUG" | "INFO" | "WARNING" | "ERROR";

export const startConsoleLogListen = () => {
    listen("LOG", (e) => {
        const msg = e.payload as LogPayload;
        if (msg.target !== "progress" && !msg.target?.startsWith("game")) {
            switch (msg.logType as LogType) {
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
