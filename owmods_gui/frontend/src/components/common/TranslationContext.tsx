import english from "@assets/translations/english.json";
import japanese from "@assets/translations/japanese.json";
import chinese from "@assets/translations/chinese.json";
import vietnamese from "@assets/translations/vietnamese.json";
import wario from "@assets/translations/wario.json";
import french from "@assets/translations/french.json";
//import portugueseBr from "@assets/translations/portuguese-br.json";
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
    Japanese: japanese,
    Chinese: chinese,
    Vietnamese: vietnamese,
    Wario: wario,
    French: french
};

export const TranslationNameMap = {
    Vietnamese: "Tiếng Việt",
    Japanese: "日本語",
    Chinese: "汉语"
    //    BrazilianPortuguese: "Brazilian Portuguese"
};

export const TranslationContext = createContext<Language>(Language.English);
