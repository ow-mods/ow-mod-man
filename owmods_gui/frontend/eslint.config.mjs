import { fixupConfigRules, fixupPluginRules } from "@eslint/compat";
import react from "eslint-plugin-react";
import typescriptEslint from "@typescript-eslint/eslint-plugin";
import globals from "globals";
import tsParser from "@typescript-eslint/parser";
import path from "node:path";
import { fileURLToPath } from "node:url";
import js from "@eslint/js";
import { FlatCompat } from "@eslint/eslintrc";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const compat = new FlatCompat({
    baseDirectory: __dirname,
    recommendedConfig: js.configs.recommended,
    allConfig: js.configs.all
});

export default [
    ...fixupConfigRules(
        compat.extends(
            "eslint:recommended",
            "plugin:react/recommended",
            "plugin:react/jsx-runtime",
            "plugin:react-hooks/recommended",
            "plugin:@typescript-eslint/recommended"
        )
    ),
    {
        plugins: {
            react: fixupPluginRules(react),
            "@typescript-eslint": fixupPluginRules(typescriptEslint)
        },

        languageOptions: {
            globals: {
                ...globals.browser
            },

            parser: tsParser,
            ecmaVersion: "latest",
            sourceType: "module"
        },

        settings: {
            react: {
                version: "detect"
            }
        },

        rules: {
            "@typescript-eslint/no-non-null-assertion": "off"
        }
    }
];
