import { hooks } from "@commands";
import { SocketMessageType } from "@types";
import { CSSProperties, MutableRefObject, memo, useEffect, useMemo } from "react";
import { VirtuosoHandle } from "react-virtuoso";

export interface LogLineProps {
    port: number;
    line: number;
    count: number;
    style?: CSSProperties;
    virtuosoRef?: MutableRefObject<VirtuosoHandle | null>;
}

const LogLine = memo((props: LogLineProps) => {
    const [status, msg, err] = hooks.getLogLine("", { port: props.port, line: props.line });

    const messageType = useMemo(() => {
        return Object.keys(SocketMessageType)[(msg?.message.messageType as unknown as number) ?? 0];
    }, [msg?.message.messageType]);

    const msgClassName = messageType.toLowerCase();

    const senderName = msg?.message.senderName ?? "Unknown";
    const senderType = msg?.message.senderType ?? "Unknown";

    const messageLines = useMemo(
        () => (msg?.message.message ?? "").split("\n"),
        [msg?.message.message]
    );

    useEffect(() => {
        props.virtuosoRef?.current?.autoscrollToBottom?.();
    }, [status, props.virtuosoRef]);

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
                <span className={`message type-${msgClassName}`}>
                    {messageLines.map((line, i) => (
                        <div key={`${i}-${line}`}>{line}</div>
                    ))}
                </span>
                {props.count > 1 && <span className="count">{props.count}</span>}
            </div>
        );
    }
});

export default LogLine;
