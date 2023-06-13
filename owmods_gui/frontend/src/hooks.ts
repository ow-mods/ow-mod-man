import { listen } from "@tauri-apps/api/event";
import { useCallback, useContext, useEffect, useMemo, useState } from "react";
import {
    TranslationContext,
    TranslationMap,
    type TranslationKey
} from "@components/common/TranslationContext";
import { LocalMod, RemoteMod, UnsafeLocalMod } from "@types";

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
        if (status !== "Loading") {
            for (const eventToSubscribe of events) {
                listen(eventToSubscribe, () => setStatus("Loading")).catch((e) => {
                    setStatus("Error");
                    setError(e);
                });
            }
        } else {
            commandFn()
                .then((data) => {
                    setData(data as T);
                    setStatus("Done");
                })
                .catch((e) => {
                    setError(e as string);
                    setStatus("Error");
                });
        }
    }, [commandFn, events, status]);

    useEffect(() => {
        if (status === "Done") {
            setStatus("Loading");
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [...Object.values(payload ?? [])]);

    return [status, data, error];
};

export const useGetTranslation = () => {
    const context = useContext(TranslationContext);
    return useCallback(
        (key: TranslationKey, variables?: Record<string, string>) => {
            const activeTable = TranslationMap[context];
            let translated = activeTable[key];
            if (translated === undefined) {
                translated = activeTable["_"];
                const fallback = TranslationMap["English"][key] ?? "INVALID KEY: $key$";
                translated = translated.replaceAll(`$fallback$`, fallback);
                translated = translated.replaceAll(`$key$`, key);
            } else {
                for (const k in variables) {
                    translated = translated.replaceAll(`$${k}$`, variables[k]);
                }
            }
            return translated;
        },
        [context]
    );
};

export function useDebounce<TValue>(value: TValue, delayMs: number): TValue {
    const [debouncedValue, setDebouncedValue] = useState<TValue>(value);

    useEffect(() => {
        const handler = setTimeout(() => {
            setDebouncedValue(value);
        }, delayMs);

        return () => {
            clearTimeout(handler);
        };
    }, [value, delayMs]);

    return debouncedValue;
}

export interface UnifiedMod {
    name: string;
    author: string;
    description: string | undefined;
    version: string;
    enabled: boolean;
    outdated: boolean;
}

const safeOrNull = (mod: UnsafeLocalMod | null) => {
    if (mod === null) return null;
    if (mod.loadState === "invalid") {
        return null;
    } else {
        return mod.mod as LocalMod;
    }
};

export function useUnifiedMod(local: UnsafeLocalMod | null, remote: RemoteMod | null) {
    const name = useMemo(
        () => remote?.name ?? safeOrNull(local)?.manifest.name ?? "",
        [local, remote]
    );
    const author = useMemo(
        () => remote?.authorDisplay ?? remote?.author ?? safeOrNull(local)?.manifest.author ?? "",
        [local, remote]
    );

    const description = remote?.description;

    const version = useMemo(() => safeOrNull(local)?.manifest.version ?? "—", [local]);

    const enabled = safeOrNull(local)?.enabled ?? false;

    const outdated = useMemo(
        () =>
            safeOrNull(local)?.errors.filter((e) => e.errorType === "Outdated").length !== 0 ??
            false,
        [local]
    );
    return {
        name,
        author,
        description,
        version,
        enabled,
        outdated
    } as UnifiedMod;
}
