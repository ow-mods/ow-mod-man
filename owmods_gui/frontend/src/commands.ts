import { LoadState, useTauri } from "@hooks";
import { invoke } from "@tauri-apps/api/core";
import * as dialog from "@tauri-apps/plugin-dialog";
import {
    Config,
    GuiConfig,
    OWMLConfig,
    GameMessage,
    UnsafeLocalMod,
    Alert,
    ProgressBars,
    ProgressBar,
    Event,
    RemoteModOption
} from "@types";

type CommandInfo<P, R> = [P, R];
type GetCommand<V> = CommandInfo<Record<string, never>, V>;
type EmptyCommand = GetCommand<void>;
type ActionCommand<P> = CommandInfo<P, void>;
type ModCommand<M> = CommandInfo<{ uniqueName: string }, M>;
type ModAction = ModCommand<void>;

// This is a rly rly weird system where I tag a string with a type that very much isn't a string
// but hey it works and it has a minimal runtime footprint

const $ = <T>(cmd: string) => cmd as T;

const commandInfo = {
    initialSetup: $<EmptyCommand>("initial_setup"),
    refreshLocalDb: $<EmptyCommand>("refresh_local_db"),
    refreshRemoteDb: $<EmptyCommand>("refresh_remote_db"),
    getConfig: $<GetCommand<Config>>("get_config"),
    getGuiConfig: $<GetCommand<GuiConfig>>("get_gui_config"),
    getOwmlConfig: $<GetCommand<OWMLConfig>>("get_owml_config"),
    getDefaultConfigs: $<GetCommand<[Config, GuiConfig, OWMLConfig]>>("get_defaults"),
    getLocalMods: $<CommandInfo<{ filter: string; tags: string[] }, string[]>>("get_local_mods"),
    getRemoteMods:
        $<CommandInfo<{ filter: string; tags: string[] }, string[] | undefined>>("get_remote_mods"),
    getUpdatableMods: $<CommandInfo<{ filter: string }, string[]>>("get_updatable_mods"),
    getLocalMod: $<ModCommand<UnsafeLocalMod>>("get_local_mod"),
    getRemoteMod: $<ModCommand<RemoteModOption>>("get_remote_mod"),
    getLogLine: $<CommandInfo<{ port: number; line: number }, GameMessage>>("get_game_message"),
    toggleMod:
        $<CommandInfo<{ uniqueName: string; enabled: boolean; recursive: boolean }, string[]>>(
            "toggle_mod"
        ),
    toggleAll: $<CommandInfo<{ enabled: boolean }, string[]>>("toggle_all"),
    openModFolder: $<ModAction>("open_mod_folder"),
    openModGithub: $<ModAction>("open_mod_github"),
    openModReadme: $<ModAction>("open_mod_readme"),
    openOwml: $<EmptyCommand>("open_owml"),
    uninstallMod: $<ModCommand<string[]>>("uninstall_mod"),
    uninstallBrokenMod: $<ActionCommand<{ modPath: string }>>("uninstall_broken_mod"),
    installMod: $<CommandInfo<{ uniqueName: string; prerelease?: boolean }, void>>("install_mod"),
    installUrl: $<ActionCommand<{ url: string }>>("install_url"),
    installZip: $<ActionCommand<{ path: string }>>("install_zip"),
    installOwml: $<ActionCommand<{ prerelease: boolean }>>("install_owml"),
    setOwml: $<CommandInfo<{ path: string }, boolean>>("set_owml"),
    saveConfig: $<ActionCommand<{ config: Config }>>("save_config"),
    saveGuiConfig: $<ActionCommand<{ guiConfig: GuiConfig }>>("save_gui_config"),
    saveOwmlConfig: $<ActionCommand<{ owmlConfig: OWMLConfig }>>("save_owml_config"),
    updateMod: $<ModAction>("update_mod"),
    updateAll: $<ActionCommand<{ uniqueNames: string[] }>>("update_all_mods"),
    logsAreActive: $<CommandInfo<{ port: number }, boolean>>("active_log"),
    startLogs: $<EmptyCommand>("start_logs"),
    runGame: $<EmptyCommand>("run_game"),
    clearLogs: $<ActionCommand<{ port: number }>>("clear_logs"),
    getLogLines: $<
        CommandInfo<
            {
                port: number;
                filterType?: number | undefined;
                search: string;
            },
            [number[], number]
        >
    >("get_log_lines"),
    exportMods: $<ActionCommand<{ path: string }>>("export_mods"),
    importMods: $<ActionCommand<{ path: string; disableMissing: boolean }>>("import_mods"),
    fixDeps: $<ActionCommand<{ uniqueName: string }>>("fix_mod_deps"),
    checkDBForIssues: $<GetCommand<boolean>>("db_has_issues"),
    getAlert: $<GetCommand<Alert>>("get_alert"),
    dismissAlert: $<ActionCommand<{ alert: Alert }>>("dismiss_alert"),
    popProtocolURL: $<ActionCommand<{ id: string }>>("pop_protocol_url"),
    checkOWML: $<GetCommand<boolean>>("check_owml"),
    getDownloads: $<GetCommand<ProgressBars>>("get_downloads"),
    clearDownloads: $<EmptyCommand>("clear_downloads"),
    getBarByUniqueName: $<ModCommand<ProgressBar>>("get_bar_by_unique_name"),
    getBusyMods: $<GetCommand<string[]>>("get_busy_mods"),
    getModBusy: $<ModCommand<boolean>>("get_mod_busy"),
    hasDisabledDeps: $<ModCommand<boolean>>("has_disabled_deps"),
    registerDropHandler: $<EmptyCommand>("register_drop_handler"),
    getDbTags: $<GetCommand<string[]>>("get_db_tags"),
    logError: $<ActionCommand<{ err: string }>>("log_error"),
    forceLogUpdate: $<ActionCommand<{ port: number }>>("force_log_update"),
    showLogsFolder: $<EmptyCommand>("show_log_folder")
};

type Command = keyof typeof commandInfo;

const makeInvoke = (key: Command, forceNoDisplayErr?: boolean) => {
    const name = commandInfo[key];
    return (payload?: (typeof name)[0], displayErr?: boolean) => {
        const promise = invoke(name as unknown as string, payload ?? {}) as Promise<
            (typeof name)[1]
        >;
        if (!(forceNoDisplayErr ?? false) && (displayErr ?? true)) {
            promise.catch((e) => {
                dialog.message(e, { kind: "error", title: `Error (${name})` });
            });
        }
        return promise;
    };
};

const makeHook = (key: Command) => {
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const name = commandInfo[key];
    return <E extends Event["name"]>(
        eventName: E | Event["name"][],
        payload?: (typeof name)[0]
    ) => {
        const fn = makeInvoke(key, true);
        return useTauri<(typeof name)[1], E>(
            eventName,
            () => fn(payload ?? {}) as unknown as Promise<(typeof name)[1]>,
            payload
        );
    };
};

export type Commands = {
    [T in Command]: (
        payload?: (typeof commandInfo)[T][0],
        displayErr?: boolean
    ) => Promise<(typeof commandInfo)[T][1]>;
};

export type Hooks = {
    [T in Command]: (
        eventName: Event["name"] | Event["name"][],
        payload?: (typeof commandInfo)[T][0],
        shouldChangeFn?: (params: unknown) => boolean
    ) => [LoadState, (typeof commandInfo)[T][1] | null, Error | null];
};

const _commands: Record<string, unknown> = {};
const _hooks: Record<string, unknown> = {};

for (const k of Object.keys(commandInfo)) {
    _commands[k as Command] = makeInvoke(k as Command);
    _hooks[k as Command] = makeHook(k as Command);
}

/**
 * Run a command with the given payload
 */
export const commands = _commands as Commands;

/**
 * Subscribe to an event and run the command on that event
 */
export const hooks = _hooks as Hooks;
