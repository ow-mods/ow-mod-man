import english from "@assets/translations/english.json";
import wario from "@assets/translations/wario.json";
import portugueseBr from "@assets/translations/portuguese-br.json";
import { Language } from "@types";
import { createContext } from "react";

type Translations = Record<string, string>;

export const TranslationMap: Record<Language, Translations> = {
    English: english,
    BrazilianPortuguese: portugueseBr,
    Wario: wario
};

export const TranslationNameMap = {
    BrazilianPortuguese: "Brazilian Portuguese"
};

export const TranslationContext = createContext<Language>(Language.English);
