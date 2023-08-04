import { TestContext, vi } from "vitest";

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
