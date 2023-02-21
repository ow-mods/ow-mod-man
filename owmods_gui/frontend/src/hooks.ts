import { invoke } from "@tauri-apps/api";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useContext, useEffect, useMemo, useRef, useState } from "react";
import { TranslationContext, TranslationMap } from "@components/TranslationContext";
import { Config, GuiConfig } from "@types";

export type LoadState = "Loading" | "Done" | "Error";

const subscribeTauri = (name: string) => {
    return async (callback: (p: unknown) => void) => {
        return await listen(name, callback);
    };
};

const getTauriSnapshot = <T>(cmdName: string, payload: unknown): (() => Promise<T>) => {
    return async () => {
        return await invoke(cmdName, payload as Record<string, unknown>);
    };
};

export const useTauri = <T>(
    eventName: string | string[],
    commandName: string,
    commandPayload?: unknown
): [LoadState, T | null, string | null] => {
    const [status, setStatus] = useState<LoadState>("Loading");
    const [data, setData] = useState<T | null>(null);
    const [error, setError] = useState<string | null>(null);
    const events = useMemo(() => (Array.isArray(eventName) ? eventName : [eventName]), [eventName]);
    let unsubscribe: UnlistenFn | null = null;

    useEffect(() => {
        if (status !== "Loading") {
            for (const eventToSubscribe of events) {
                subscribeTauri(eventToSubscribe)(() => setStatus("Loading"))
                    .then((u) => {
                        unsubscribe = u;
                    })
                    .catch((e) => {
                        setStatus("Error");
                        setError(e);
                    });
            }
        } else {
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

export const useTauriCount = (incEvent: string, decEvent: string, initial?: number) => {
    const [count, setCount] = useState(initial ?? 0);

    const countRef = useRef(initial ?? 0);

    const incCount = () => setCount(countRef.current + 1);
    const decCount = () => setCount(countRef.current - 1);

    useEffect(() => {
        countRef.current = count;
    }, [count]);

    useEffect(() => {
        subscribeTauri(incEvent)(incCount).catch(console.warn);
        subscribeTauri(decEvent)(decCount).catch(console.warn);
    }, []);

    return count;
};

export const useTranslation = (key: string, variables?: Record<string, string>) => {
    const context = useContext(TranslationContext);
    return useMemo(() => {
        const activeTable = TranslationMap[context];
        let translated = activeTable[key] ?? activeTable["_"];
        for (const k in variables) {
            translated = translated.replaceAll(`$${k}$`, variables[k]);
        }
        return translated;
    }, [context, key, variables]);
};

export const useTranslations = (keys: string[]) => {
    return keys.map((k) => useTranslation(k));
};

export const useConfig = () => useTauri<Config>("CONFIG_RELOAD", "fetch_config");
export const useGuiConfig = () => useTauri<GuiConfig>("GUI_CONFIG_RELOAD", "get_gui_config");
