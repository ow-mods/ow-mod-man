import { hooks } from "@commands";
import { SocketMessageType } from "@types";
import { CSSProperties, memo, useMemo } from "react";

export interface LogLineProps {
    port: number;
    line: number;
    count: number;
    style?: CSSProperties;
}

const LogLine = memo((props: LogLineProps) => {
    const [status, msg, err] = hooks.getLogLine("", { port: props.port, line: props.line });

    const messageType = useMemo(() => {
        return Object.keys(SocketMessageType)[(msg?.message.messageType as unknown as number) ?? 0];
    }, [msg?.message.messageType]);

    const msgClassName = messageType.toLowerCase();

    const senderName = msg?.message.senderName ?? "Unknown";
    const senderType = msg?.message.senderType ?? "Unknown";

    if (status === "Error") {
        return <p className="log-line center">{err!.toString()}</p>;
    } else {
        return (
            <div className="log-line">
                <span
                    className="sender"
                    data-tooltip={`${senderName}::${senderType}`}
                    data-placement="right"
                >
                    <span>{senderName}</span>
                </span>
                <span className={`message type-${msgClassName}`}>{msg?.message.message}</span>
                {props.count > 1 && <span className="count">{props.count}</span>}
            </div>
        );
    }
});

export default LogLine;
