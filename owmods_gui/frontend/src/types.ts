// Defines main types for the app.
// Most of these types are also defined in owmods_core/mods.rs, those structs and these type should be kept in sync as they're serialized and deserialized through tauri's IPC.
// Same goes for config.rs

export interface LocalMod {
    enabled: boolean;
    modPath: string;
    manifest: ModManifest;
}

export interface ModManifest {
    uniqueName: string;
    name: string;
    author: string;
    version: string;
    owmlVersion?: string;
    dependencies?: string[];
    conflicts?: string[];
    pathsToPreserve?: string[];
}

export interface RemoteMod {
    downloadUrl: string;
    downloadCount: number;
    version: string;
    name: string;
    uniqueName: string;
    description: string;
    readme?: ModReadMe;
    required?: boolean;
    repo: string;
    author: string;
    authorDisplay?: string;
    parent?: string;
    prerelease?: ModPrerelease;
    alpha?: boolean;
    tags?: string[];
}

export interface ModPrerelease {
    downloadUrl: string;
    version: string;
}

export interface ModReadMe {
    htmlUrl: string;
    downloadUrl: string;
}

export interface Config {
    alertUrl: string;
    databaseUrl: string;
    owmlPath: string;
    winePrefix: string;
}

export interface OwmlConfig {
    gamePath: string;
    debugMode: boolean;
    incrementalGC: boolean;
    forceExe: boolean;
}

export const ThemeArr = [
    "White",
    "Blue",
    "Pink",
    "Green",
    "Yellow",
    "Orange",
    "Blurple",
    "GhostlyGreen"
] as const;
export const LanguageArr = ["English", "Wario"] as const;

export type Theme = (typeof ThemeArr)[number];
export type Language = (typeof LanguageArr)[number];

export interface GuiConfig {
    theme: Theme;
    rainbow: boolean;
    language: Language;
    watchFs: boolean;
}
