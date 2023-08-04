import { describe, expect, it } from "vitest";
import { render, screen, waitForElementToBeRemoved } from "@testing-library/react";
import { act } from "react-dom/test-utils";
import { getTestTranslation, throwTestError } from "../utils";
import { emit } from "@events";
import OwmlModal from "@components/main/OwmlModal";

const setupText = getTestTranslation("OWML_SETUP_MESSAGE");
const cancelText = getTestTranslation("CANCEL");
const continueText = getTestTranslation("CONTINUE");

describe("OWML Setup Modal Test", () => {
    it("Should open if OWML is not installed, and not show the cancel button", async (ctx) => {
        ctx.commandFns.check_owml = () => false;

        render(<OwmlModal />);

        await act(async () => {
            await emit("owmlConfigReload", undefined);
        });

        expect(await screen.findByText(setupText)).toBeDefined();
        expect(await screen.queryByText(cancelText)).toBeNull();
    });

    it("Should not open if OWML is installed", async (ctx) => {
        ctx.commandFns.check_owml = () => true;

        render(<OwmlModal />);

        await act(async () => {
            await emit("owmlConfigReload", undefined);
        });

        expect(await screen.queryByText(setupText)).toBeNull();
    });

    it("Should open when openOwmlSetup is emitted, and show the cancel button", async (ctx) => {
        ctx.commandFns.check_owml = () => true;

        render(<OwmlModal />);

        await act(async () => {
            await emit("openOwmlSetup", undefined);
        });

        expect(await screen.findByText(setupText)).toBeDefined();
        expect(await screen.findByText(cancelText)).toBeDefined();
    });

    it("Should close when cancel is clicked", async (ctx) => {
        ctx.commandFns.check_owml = () => true;

        render(<OwmlModal />);

        await act(async () => {
            await emit("openOwmlSetup", undefined);
        });

        expect(await screen.findByText(setupText)).toBeDefined();
        const button = await screen.findByText(cancelText);
        expect(button).toBeDefined();

        act(() => {
            button.click();
        });

        await waitForElementToBeRemoved(() => screen.queryByText(setupText), {
            onTimeout: (e) => {
                throw e;
            }
        });
    });

    it("Should close when OWML is installed", async (ctx) => {
        ctx.commandFns.check_owml = () => false;
        ctx.commandFns.install_owml = () => {
            ctx.commandFns.check_owml = () => true;
            return emit("owmlConfigReload", undefined);
        };

        render(<OwmlModal />);

        await act(async () => {
            await emit("openOwmlSetup", undefined);
        });

        expect(await screen.findByText(setupText)).toBeDefined();

        const button = await screen.findByText(continueText);

        act(() => {
            button.click();
        });

        await waitForElementToBeRemoved(() => screen.queryByText(setupText), {
            onTimeout: (e) => {
                throw e;
            }
        });
    });

    it("Should not close if there is an error installing OWML", async (ctx) => {
        ctx.commandFns.check_owml = () => false;
        ctx.commandFns.install_owml = async () => throwTestError();

        render(<OwmlModal />);

        await act(async () => {
            await emit("openOwmlSetup", undefined);
        });

        expect(await screen.findByText(setupText)).toBeDefined();

        const button = await screen.findByText(continueText);

        act(() => {
            button.click();
        });

        expect(await screen.findByText(setupText)).toBeDefined();
    });
});
