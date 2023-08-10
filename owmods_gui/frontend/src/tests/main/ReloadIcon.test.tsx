import { withStyledErrorBoundary } from "@components/common/StyledErrorBoundary";
import _ReloadIcon from "@components/main/top-bar/ReloadIcon";
import { emit } from "@events";
import { act, render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

const ReloadIcon = withStyledErrorBoundary(_ReloadIcon, { justHide: true });

describe("Reload Icon", () => {
    it("Should Refresh Everything On Click", async (ctx) => {
        ctx.commandFns.refresh_local_db = vi.fn();
        ctx.commandFns.initial_setup = vi.fn();
        ctx.commandFns.refresh_remote_db = vi.fn();

        render(<ReloadIcon />);

        const refreshIcon = (await screen.findAllByLabelText("Refresh"))[0];

        expect(refreshIcon).toBeDefined();

        await act(async () => {
            refreshIcon.click();
        });

        expect(ctx.commandFns.refresh_local_db).toHaveBeenCalled();
        expect(ctx.commandFns.initial_setup).toHaveBeenCalled();
        expect(ctx.commandFns.refresh_remote_db).toHaveBeenCalled();
    });

    it("Should Refresh The Local DB When Requested", async (ctx) => {
        ctx.commandFns.refresh_local_db = vi.fn();

        render(<ReloadIcon />);

        const refreshIcon = (await screen.findAllByLabelText("Refresh"))[0];

        expect(refreshIcon).toBeDefined();

        await act(async () => {
            await emit("requestReload", "LOCAL");
        });

        waitFor(() => {
            expect(ctx.commandFns.refresh_local_db).toHaveBeenCalled();
        });
    });

    it("Should Refresh The GUI and Normal Config When Requested", async (ctx) => {
        ctx.commandFns.initial_setup = vi.fn();

        render(<ReloadIcon />);

        const refreshIcon = (await screen.findAllByLabelText("Refresh"))[0];

        expect(refreshIcon).toBeDefined();

        await act(async () => {
            await emit("requestReload", "GUI");
        });

        waitFor(() => {
            expect(ctx.commandFns.initial_setup).toHaveBeenCalled();
        });

        await act(async () => {
            await emit("requestReload", "CONFIG");
        });

        waitFor(() => {
            expect(ctx.commandFns.initial_setup).toHaveBeenCalled();
        });
    });
});
