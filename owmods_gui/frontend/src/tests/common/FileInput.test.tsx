import { OpenFileInput } from "@components/common/FileInput";
import { render, screen, act } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { getTestTranslation } from "../utils";

const browseText = getTestTranslation("BROWSE");

describe("File Input", () => {
    it("Should update the value when a file is selected", async (ctx) => {
        let value = "";

        const testFileName = "/test/dir/test_file.txt";

        ctx.tauriFns["Dialog"] = {
            openDialog: () => testFileName
        };

        render(
            <OpenFileInput
                value={value}
                onChange={(newVal) => {
                    value = newVal;
                }}
                dialogOptions={{ title: "Test Select" }}
                id="test-file-select"
            />
        );

        const button = await screen.findByText(browseText);

        act(() => {
            button.click();
        });

        expect(await screen.findAllByText(browseText)).toBeDefined();

        expect(value).toBe(testFileName);
    });
});
