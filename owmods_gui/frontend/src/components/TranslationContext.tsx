import english from "@assets/translations/english.json";
import wario from "@assets/translations/wario.json";
import { createContext } from "react";

type Translations = Record<string, string>;

export const TranslationMap: Record<string, Translations> = {
    English: english,
    Wario: wario,
    _: { _: "Unsupported Language" }
};

export const TranslationContext = createContext("English");
