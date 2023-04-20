import { hooks } from "@commands";
import { SocketMessageType } from "@types";
import { CSSProperties, memo, useEffect, useMemo, useRef } from "react";

export interface LogLineProps {
    port: number;
    index: number;
    line: number;
    count: number;
    style: CSSProperties;
    reportSize?: (i: number, l: number, size: number) => void;
}

const LogLine = memo(
    (props: LogLineProps) => {
        const divRef = useRef<HTMLDivElement>(null);
        const [status, msg, err] = hooks.getLogLine("", { port: props.port, line: props.line });

        const messageType = useMemo(() => {
            return Object.keys(SocketMessageType)[
                (msg?.message.messageType as unknown as number) ?? 0
            ];
        }, [msg?.message.messageType]);

        useEffect(() => {
            if (divRef.current) {
                props.reportSize?.(props.index, props.line, divRef.current.offsetHeight);
            }
        }, [msg?.message, divRef.current !== null]);

        const msgClassName = messageType.toLowerCase();

        if (status === "Error") {
            return <p className="center">{err!.toString()}</p>;
        } else {
            return (
                <div style={props.style}>
                    <div ref={divRef} className="log-line">
                        <span className="sender">{msg?.message.senderName ?? "Unknown"}</span>
                        <span className={`message type-${msgClassName}`}>
                            {msg?.message.message}
                        </span>
                        {props.count > 1 && <span className="count">{props.count}</span>}
                    </div>
                </div>
            );
        }
    },
    (current, next) =>
        current.line === next.line &&
        current.count === next.count &&
        current.style.display === next.style.display &&
        current.style.height === next.style.height &&
        current.style.top === next.style.top
);

export default LogLine;
