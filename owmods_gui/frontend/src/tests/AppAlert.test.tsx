import { beforeAll, describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";

import AppAlert from "@components/main/AppAlert";
import { Alert } from "@types";
import { expectShellInvoke, throwTestError } from "./utils";
import { act } from "react-dom/test-utils";

beforeAll(() => {
    vi.mock("@events");
});

const baseAlert: Alert = {
    enabled: true,
    severity: "error",
    message: "Test Alert"
};

describe("AppAlert test", () => {
    it("Should render an alert", async (ctx) => {
        ctx.commandFns.get_alert = () => baseAlert;

        render(<AppAlert />);

        expect(await screen.findByText("Test Alert")).toBeDefined();
        expect(await screen.queryByText("More Info")).toBeNull();
    });

    it("Should render an alert with a URL and open the URL on click", async (ctx) => {
        const shellFn = expectShellInvoke(ctx, "https://outerwildsmods.com");

        ctx.commandFns.get_alert = () => ({
            ...baseAlert,
            url: "https://outerwildsmods.com",
            urlLabel: "Website"
        });

        render(<AppAlert />);

        expect(await screen.findByText("Test Alert")).toBeDefined();
        const button = await screen.findByText("Website");
        expect(button).toBeDefined();

        act(() => {
            button.click();
        });

        expect(shellFn).toHaveBeenCalledWith("https://outerwildsmods.com");
    });

    it("Should hide the alert when disabled", async (ctx) => {
        ctx.commandFns.get_alert = () => ({
            ...baseAlert,
            enabled: false
        });

        render(<AppAlert />);

        await act(async () => {
            expect(await screen.queryByText("Test Alert")).toBeNull();
        });
    });

    it("Should hide the alert if an error is thrown", async (ctx) => {
        ctx.commandFns.get_alert = () => {
            throwTestError();
        };

        render(<AppAlert />);

        await act(async () => {
            expect(await screen.queryByText("Test Alert")).toBeNull();
        });
    });
});
