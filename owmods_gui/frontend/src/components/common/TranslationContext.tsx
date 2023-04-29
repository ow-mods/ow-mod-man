import english from "@assets/translations/english.json";
import wario from "@assets/translations/wario.json";
import portugueseBr from "@assets/translations/portuguese-br.json";
import { Language } from "@types";
import { createContext } from "react";

// English is used as the base language that defines the available translation keys.
// "_" is the only mandatory key.
interface Translations extends Partial<typeof english> {
    _: string;
}

export type TranslationKey = keyof Translations;

export const TranslationMap: Record<Language, Translations> = {
    English: english,
    BrazilianPortuguese: portugueseBr,
    Wario: wario
};

export const TranslationNameMap = {
    BrazilianPortuguese: "Brazilian Portuguese"
};

export const TranslationContext = createContext<Language>(Language.English);
