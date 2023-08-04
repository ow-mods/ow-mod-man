/* eslint-disable @typescript-eslint/ban-ts-comment */
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { afterEach, beforeAll, beforeEach } from "vitest";
import { randomFillSync } from "crypto";

declare module "vitest" {
    export interface TestContext {
        commandFns: Record<string, (args: unknown) => unknown>;
        tauriFns: Record<string, Record<string, (args: unknown) => unknown>>;
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
});

beforeEach((ctx) => {
    ctx.commandFns = {};
    ctx.tauriFns = {};
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
