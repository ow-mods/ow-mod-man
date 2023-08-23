import { TranslationKey } from "@components/common/TranslationContext";
import { staticGetTranslation } from "@hooks";
import { Language } from "@types";
import { TestContext, vi } from "vitest";

export const getTestTranslation = (key: TranslationKey) => {
    return staticGetTranslation(Language.English, key);
};

export const expectShellInvoke = (ctx: TestContext, url: string) => {
    const fn = vi.fn();
    if (!ctx.tauriFns["Shell"]) {
        ctx.tauriFns["Shell"] = {};
    }
    ctx.tauriFns["Shell"]["open"] = () => {
        fn(url);
    };
    return fn;
};

export const throwTestError = (message?: string) => {
    throw new Error(`~~TEST_ERROR~~ ${message ?? ""}`);
};
