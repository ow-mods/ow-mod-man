import { commands } from "@commands";
import { SocketMessage, SocketMessageType } from "@types";
import { CSSProperties, memo, useEffect, useMemo, useRef, useState } from "react";
import { LogFilter } from "./App";

export interface LogLineProps {
    port: number;
    line: number;
    style: CSSProperties;
    activeFilter: LogFilter;
    cachedData?: SocketMessage;
    reportSize?: (i: number, size: number) => void;
    getCached?: (i: number) => SocketMessage;
    remember?: (i: number, msg: SocketMessage) => void;
}

const LogLine = memo(
    (props: LogLineProps) => {
        const divRef = useRef<HTMLDivElement>(null);
        const cached = props.getCached?.(props.line);
        const state = useState<SocketMessage | null>(null);
        const msg = cached ?? state[0];

        const messageType = useMemo(() => {
            return Object.keys(SocketMessageType)[(msg?.messageType as unknown as number) ?? 0];
        }, [msg?.messageType]);

        useEffect(() => {
            if (props.activeFilter !== "Any" && messageType !== props.activeFilter) {
                props.reportSize?.(props.line, 0);
                if (divRef.current) {
                    divRef.current.style.display = "none";
                }
                console.debug("NARP!");
            }
        }, [msg?.messageType, props.activeFilter]);

        useEffect(() => {
            if (cached === undefined) {
                commands
                    .getLogLine(props)
                    .then((msg) => {
                        props.remember?.(props.line, msg);
                        state[1](msg);
                    })
                    .catch(console.error);
            }
        }, []);

        useEffect(() => {
            if (divRef.current) {
                props.reportSize?.(props.line, divRef.current.offsetHeight);
            }
        }, [msg?.message, divRef.current !== null]);

        const msgClassName = messageType.toLowerCase();

        return (
            <div style={props.style}>
                <div ref={divRef} className="log-line">
                    <span className="sender">{msg?.senderName ?? "Unknown"}</span>
                    <span className={`message type-${msgClassName}`}>{msg?.message}</span>
                </div>
            </div>
        );
    },
    (current, next) =>
        current.line === next.line &&
        current.style.display === next.style.display &&
        current.style.height === next.style.height &&
        current.style.top === next.style.top &&
        current.activeFilter === next.activeFilter
);

export default LogLine;
