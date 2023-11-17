import { useCallback, useContext, useEffect, useMemo, useState } from "react";
import {
    TranslationContext,
    TranslationMap,
    type TranslationKey
} from "@components/common/TranslationContext";
import { Event, FailedMod, Language, LocalMod, RemoteMod, UnsafeLocalMod } from "@types";
import { useErrorBoundary } from "react-error-boundary";
import { Params, listen } from "@events";

export type LoadState = "Loading" | "Done";

/**
 * Use @commands:hooks if possible
 */
export const useTauri = <T, E extends Event["name"]>(
    eventName: Event["name"] | Event["name"][],
    commandFn: () => Promise<T>,
    payload: unknown,
    shouldChangeFn?: (params: Params<E>) => boolean
): [LoadState, T | null] => {
    const [status, setStatus] = useState<LoadState>("Loading");
    const [data, setData] = useState<T | null>(null);
    const events = useMemo(() => (Array.isArray(eventName) ? eventName : [eventName]), [eventName]);

    const errorBound = useErrorBoundary();

    useEffect(() => {
        if (status !== "Loading") {
            for (const eventToSubscribe of events) {
                listen(eventToSubscribe, (params) => {
                    if (shouldChangeFn && !shouldChangeFn(params)) return;
                    setStatus("Loading");
                });
            }
        } else {
            commandFn()
                .then((data) => {
                    setData(data as T);
                    errorBound.resetBoundary();
                })
                .catch((e) => {
                    errorBound.showBoundary(e);
                })
                .finally(() => setStatus("Done"));
        }
    }, [commandFn, shouldChangeFn, errorBound, events, status]);

    useEffect(() => {
        if (status === "Done") {
            setStatus("Loading");
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [...Object.values(payload ?? [])]);

    return [status, data];
};

export const staticGetTranslation = (
    language: Language,
    key: TranslationKey,
    variables?: Record<string, string>
) => {
    const activeTable = TranslationMap[language];
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
};

export const useGetTranslation = () => {
    const context = useContext(TranslationContext);
    return useCallback(
        (key: TranslationKey, variables?: Record<string, string>) => {
            return staticGetTranslation(context, key, variables);
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
    slug?: string;
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
        () =>
            remote?.name ??
            safeOrNull(local)?.manifest.name ??
            ((local?.mod as FailedMod) ?? { displayPath: null }).displayPath ??
            "",
        [local, remote]
    );
    const author = useMemo(
        () => remote?.authorDisplay ?? remote?.author ?? safeOrNull(local)?.manifest.author ?? "—",
        [local, remote]
    );

    const description = remote?.description;

    const version = useMemo(() => safeOrNull(local)?.manifest.version ?? "—", [local]);

    const enabled = safeOrNull(local)?.enabled ?? false;

    const slug = remote?.slug;

    const outdated = useMemo(
        () => safeOrNull(local)?.errors.find((e) => e.errorType === "Outdated") ?? false,
        [local]
    );
    return {
        name,
        slug,
        author,
        description,
        version,
        enabled,
        outdated
    } as UnifiedMod;
}
