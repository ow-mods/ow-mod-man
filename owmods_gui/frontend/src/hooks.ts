import { invoke } from "@tauri-apps/api";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useContext, useEffect, useMemo, useState } from "react";
import { TranslationContext, TranslationMap } from "@components/TranslationContext";

export type LoadState = "Loading" | "Done" | "Error";

const subscribeTauri = (name: string) => {
    return async (callback: () => void) => {
        return await listen(name, callback);
    };
};

const getTauriSnapshot = <T>(cmdName: string, payload: unknown): (() => Promise<T>) => {
    return async () => {
        return await invoke(cmdName, payload as Record<string, unknown>);
    };
};

export const useTauri = <T>(
    eventName: string,
    commandName: string,
    commandPayload?: unknown
): [LoadState, T | null, string | null] => {
    const [status, setStatus] = useState<LoadState>("Loading");
    const [data, setData] = useState<T | null>(null);
    const [error, setError] = useState<string | null>(null);
    let unsubscribe: UnlistenFn | null = null;

    useEffect(() => {
        if (status !== "Loading") {
            console.debug(`Begin subscribe to ${eventName}`);
            subscribeTauri(eventName)(() => setStatus("Loading"))
                .then((u) => {
                    unsubscribe = u;
                })
                .catch((e) => {
                    setStatus("Error");
                    setError(e);
                });
        } else {
            console.debug(`${eventName} Fired, Invoking ${commandName} with args`, commandPayload);
            getTauriSnapshot(commandName, commandPayload)()
                .then((data) => {
                    setData(data as T);
                    setStatus("Done");
                })
                .catch((e) => {
                    setError(e as string);
                    setStatus("Error");
                })
                .finally(() => {
                    unsubscribe = null;
                });
        }
        return () => unsubscribe?.();
    }, [status]);

    return [status, data, error];
};

export const useTranslation = (key: string) => {
    const context = useContext(TranslationContext);
    console.debug(`Getting Translation For ${key}`);
    return useMemo(() => {
        const activeTable = TranslationMap[context] ?? TranslationMap["_"];
        return activeTable[key] ?? activeTable["_"];
    }, [key]);
};

export const useTranslations = (keys: string[]) => {
    return keys.map((k) => useTranslation(k));
};
