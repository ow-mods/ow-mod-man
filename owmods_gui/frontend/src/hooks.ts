import { listen } from "@tauri-apps/api/event";
import { useContext, useEffect, useMemo, useRef, useState } from "react";
import { TranslationContext, TranslationMap } from "@components/common/TranslationContext";
import ThemeMap from "./theme";
import { Theme } from "@types";
import rainbowTheme from "@styles/rainbow.scss?inline";

export type LoadState = "Loading" | "Done" | "Error";

/**
 * Use @commands:hooks if possible
 */
export const useTauri = <T>(
    eventName: string | string[],
    commandFn: () => Promise<T>,
    payload: unknown
): [LoadState, T | null, string | null] => {
    const [status, setStatus] = useState<LoadState>("Loading");
    const [data, setData] = useState<T | null>(null);
    const [error, setError] = useState<string | null>(null);
    const events = useMemo(() => (Array.isArray(eventName) ? eventName : [eventName]), [eventName]);

    useEffect(() => {
        let cancel = false;
        if (status !== "Loading") {
            for (const eventToSubscribe of events) {
                listen(eventToSubscribe, () => setStatus("Loading")).catch((e) => {
                    console.debug(eventName, "~~TRY_FIRE~~");
                    if (cancel) return;
                    console.debug(eventName, "~~FIRE~~");
                    setStatus("Error");
                    setError(e);
                });
            }
        } else {
            commandFn()
                .then((data) => {
                    if (cancel) return;
                    if (eventName !== "") console.debug(eventName, data);
                    setData(data as T);
                    setStatus("Done");
                })
                .catch((e) => {
                    if (cancel) return;
                    setError(e as string);
                    setStatus("Error");
                });
        }
        return () => {
            cancel = true;
        };
    }, [status]);

    useEffect(() => {
        if (status === "Done") {
            setStatus("Loading");
        }
    }, Object.values(payload ?? []));

    return [status, data, error];
};

export const useTauriCount = (incEvent: string, decEvent: string, initial?: number) => {
    const [count, setCount] = useState(initial ?? 0);

    const countRef = useRef(initial ?? 0);

    useEffect(() => {
        let cancel = false;
        listen(incEvent, () => {
            if (cancel) return;
            countRef.current++;
            setCount(countRef.current);
        }).catch(console.warn);
        listen(decEvent, () => {
            if (cancel) return;
            countRef.current--;
            setCount(countRef.current);
        }).catch(console.warn);
        return () => {
            cancel = true;
        };
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
    }, [context, key, ...Object.values(variables ?? {})]);
};

export const useTranslations = (keys: string[]) => {
    return keys.map((k) => useTranslation(k));
};

export const useTheme = (theme: Theme, rainbow: boolean) => {
    useEffect(() => {
        let newTheme = ThemeMap[theme ?? "White"];
        if (rainbow) {
            newTheme += rainbowTheme;
        }
        document.getElementById("currentTheme")!.textContent = newTheme;
    }, [theme, rainbow]);
};
