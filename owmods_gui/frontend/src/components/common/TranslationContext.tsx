import english from "@assets/translations/english.json";
import wario from "@assets/translations/wario.json";
import corby from "@assets/translations/corby.json";
import { Language } from "@types";
import { createContext } from "react";

type Translations = Record<string, string>;

export const TranslationMap: Record<Language, Translations> = {
    English: english,
    Wario: wario,
    Corby: corby
};

export const TranslationContext = createContext<Language>(Language.English);
