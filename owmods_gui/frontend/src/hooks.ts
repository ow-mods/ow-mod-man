import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

export type LoadState = "Loading" | "Done" | "Error";

const subscribeTauri = (name: string) => {
    return async (callback: () => void) => {
        return await listen(name, callback);
    };
};

const getTauriSnapshot = <T, P>(cmdName: string, payload: P): (() => Promise<T>) => {
    return async () => {
        return await invoke(cmdName, payload as Record<string, unknown>);
    };
};

export const useTauri = <T, P>(
    eventName: string,
    commandName: string,
    commandPayload?: P
): [LoadState, T | null, string | null] => {
    const [status, setStatus] = useState<LoadState>("Loading");
    const [data, setData] = useState<T | null>(null);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        let u = () => {};
        if (status !== "Loading") {
            console.debug(`Begin subscribe to ${eventName}`);
            subscribeTauri(eventName)(() => setStatus("Loading")).then((unsubscribe) => {
                u = unsubscribe;
            });
        } else {
            console.debug(`Invoking ${commandName} with args ${commandPayload ?? "null"}`);
            getTauriSnapshot(commandName, commandPayload)()
                .then((data) => {
                    setData(data as T);
                    setStatus("Done");
                })
                .catch((e) => {
                    setError(e as string);
                    setStatus("Error");
                });
        }
        return u;
    }, [status]);

    return [status, data, error];
};
