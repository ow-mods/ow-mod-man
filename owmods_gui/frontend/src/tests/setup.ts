/* eslint-disable @typescript-eslint/ban-ts-comment */
import { clearMocks, mockIPC, mockWindows } from "@tauri-apps/api/mocks";
import { afterEach, beforeAll, beforeEach, vi } from "vitest";
import { randomFillSync } from "crypto";
import * as events from "@events";
import { Event } from "@types";

declare module "vitest" {
    export interface TestContext {
        commandFns: Record<string, (args: unknown) => unknown>;
        tauriFns: Record<string, Record<string, (args: unknown) => unknown>>;
        eventFns: Partial<Record<Event["name"], ((args: unknown) => void)[]>>;
    }
}

// jsdom doesn't come with a WebCrypto implementation
beforeAll(() => {
    Object.defineProperty(window, "crypto", {
        value: {
            // @ts-ignore
            getRandomValues: (buffer) => {
                return randomFillSync(buffer);
            }
        }
    });
    mockWindows("main");
});

beforeEach((ctx) => {
    ctx.commandFns = {};
    ctx.tauriFns = {};
    ctx.eventFns = {};
    mockIPC((command, args) => {
        if (ctx.commandFns[command]) {
            return ctx.commandFns[command](args);
        } else if (command === "tauri") {
            //console.debug("tauri", args);
            const mod = args["__tauriModule"] as string;
            if (ctx.tauriFns[mod]) {
                const message = args["message"] as Record<string, unknown>;
                // @ts-ignore
                const fn = ctx.tauriFns[mod][message["cmd"]];
                if (fn) {
                    return fn(message["args"]);
                }
            }
        } else {
            throw new Error(`Unknown command ${command}`);
        }
    });
    vi.spyOn(events, "listen").mockImplementation((name, callback) => {
        if (!ctx.eventFns[name]) {
            ctx.eventFns[name] = [];
        }
        ctx.eventFns[name]!.push(callback);
        return () => {
            ctx.eventFns[name]!.splice(ctx.eventFns[name]!.indexOf(callback), 1);
        };
    });
    vi.spyOn(events, "emit").mockImplementation(async (name, params) => {
        if (ctx.eventFns[name]) {
            for (const handler of ctx.eventFns[name]!) {
                handler(params);
            }
        }
    });
});

afterEach((ctx) => {
    clearMocks();
    ctx.commandFns = {};
    ctx.tauriFns = {};
});

function onError(event: ErrorEvent) {
    // Ignore test errors
    if (event.error.toString().includes("~~TEST_ERROR~~")) {
        event.preventDefault();
    }
}

window.addEventListener("error", onError);
