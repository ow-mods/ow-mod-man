import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { act } from "react-dom/test-utils";
import _FileDrop from "@components/main/FileDrop";
import { getTestTranslation } from "../utils";
import { emit } from "@events";
import { withStyledErrorBoundary } from "@components/common/StyledErrorBoundary";

const dropText = getTestTranslation("FILE_DROP_MESSAGE");

const FileDrop = withStyledErrorBoundary(_FileDrop, { justHide: true });

describe("File Drop Test", () => {
    it("Should render on dragEnter, and not render on dragLeave", async (ctx) => {
        ctx.commandFns.check_owml = () => true;
        ctx.commandFns.register_drop_handler = () => {};

        render(<FileDrop />);

        await act(async () => {
            await emit("owmlConfigReload", undefined);
        });

        expect(await screen.queryByText(dropText)).toBeNull();

        await act(async () => {
            await emit("dragEnter", undefined);
        });

        expect(await screen.findByText(dropText)).toBeDefined();

        await act(async () => {
            await emit("dragLeave", undefined);
        });

        expect(await screen.queryByText(dropText)).toBeNull();
    });

    it("Should not render if OWML is not installed", async (ctx) => {
        ctx.commandFns.check_owml = () => false;
        ctx.commandFns.register_drop_handler = () => {};

        render(<FileDrop />);

        await act(async () => {
            await emit("owmlConfigReload", undefined);
        });

        expect(await screen.queryByText(dropText)).toBeNull();

        await act(async () => {
            await emit("dragEnter", undefined);
        });

        expect(await screen.queryByText(dropText)).toBeNull();
    });
});
