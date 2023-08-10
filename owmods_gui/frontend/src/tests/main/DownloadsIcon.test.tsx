import { describe, expect, it } from "vitest";
import { act, render, screen } from "@testing-library/react";
import _DownloadsIcon from "@components/main/top-bar/downloads/DownloadsIcon";
import { withStyledErrorBoundary } from "@components/common/StyledErrorBoundary";

const DownloadsIcon = withStyledErrorBoundary(_DownloadsIcon, { justHide: true });

describe("Downloads Icon", () => {
    it("Should Show The Number Of Active Downloads", async (ctx) => {
        ctx.commandFns.get_downloads = () => ({
            bars: {
                test: {
                    id: "test",
                    message: "Test Download",
                    progress: 50,
                    progressType: "Definite",
                    progressAction: "Download",
                    len: 100,
                    position: 0,
                    success: null
                },
                test2: {
                    id: "test2",
                    message: "Test Download 2",
                    progress: 50,
                    progressType: "Definite",
                    progressAction: "Download",
                    len: 50,
                    position: 1,
                    success: true
                }
            }
        });

        render(<DownloadsIcon />);

        expect(await screen.findByText("1")).toBeDefined();
    });

    it("Should Show The Number Of Completed Downloads", async (ctx) => {
        ctx.commandFns.get_downloads = () => ({
            bars: {
                test: {
                    id: "test",
                    message: "Test Download",
                    progress: 50,
                    progressType: "Definite",
                    progressAction: "Download",
                    len: 100,
                    position: 0,
                    success: false
                },
                test2: {
                    id: "test2",
                    message: "Test Download 2",
                    progress: 50,
                    progressType: "Definite",
                    progressAction: "Download",
                    len: 50,
                    position: 1,
                    success: true
                }
            }
        });

        render(<DownloadsIcon />);

        expect(await screen.findByText("2")).toBeDefined();
    });

    it("Should Show Stop Showing The Number Of Completed Downloads On Click", async (ctx) => {
        ctx.commandFns.get_downloads = () => ({
            bars: {
                test: {
                    id: "test",
                    message: "Test Download",
                    progress: 50,
                    progressType: "Definite",
                    progressAction: "Download",
                    len: 100,
                    position: 0,
                    success: false
                },
                test2: {
                    id: "test2",
                    message: "Test Download 2",
                    progress: 50,
                    progressType: "Definite",
                    progressAction: "Download",
                    len: 50,
                    position: 1,
                    success: true
                }
            }
        });

        render(<DownloadsIcon />);

        const button = await screen.findByLabelText("Downloads");

        expect(button).toBeDefined();

        act(() => {
            button.click();
        });

        expect(screen.queryByText("2")).toBeNull();
    });
});
