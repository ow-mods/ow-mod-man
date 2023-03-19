import { LoadState, useTauri } from "@hooks";
import { invoke } from "@tauri-apps/api";
import { Config, GuiConfig, LocalMod, OWMLConfig, RemoteMod, SocketMessage } from "@types";

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
    getLocalMods: $<CommandInfo<{ filter: string }, string[]>>("get_local_mods"),
    getRemoteMods: $<CommandInfo<{ filter: string }, string[]>>("get_remote_mods"),
    getUpdatableMods: $<GetCommand<string[]>>("get_updatable_mods"),
    getLocalMod: $<ModCommand<LocalMod>>("get_local_mod"),
    getRemoteMod: $<ModCommand<RemoteMod>>("get_remote_mod"),
    getLogLine: $<CommandInfo<{ port: number; line: number }, SocketMessage>>("get_game_message"),
    toggleMod: $<ActionCommand<{ uniqueName: string; enabled: boolean }>>("toggle_mod"),
    toggleAll: $<ActionCommand<{ enabled: boolean }>>("toggle_all"),
    openModFolder: $<ModAction>("open_mod_folder"),
    openModReadme: $<ModAction>("open_mod_readme"),
    uninstallMod: $<ModAction>("uninstall_mod"),
    installMod: $<CommandInfo<{ uniqueName: string; prerelease?: boolean }, void>>("install_mod"),
    installUrl: $<ActionCommand<{ url: string }>>("install_url"),
    installZip: $<ActionCommand<{ path: string }>>("install_zip"),
    installOwml: $<EmptyCommand>("install_owml"),
    setOwml: $<CommandInfo<{ path: string }, boolean>>("set_owml"),
    saveConfig: $<ActionCommand<{ config: Config }>>("save_config"),
    saveGuiConfig: $<ActionCommand<{ guiConfig: GuiConfig }>>("save_gui_config"),
    saveOwmlConfig: $<ActionCommand<{ owmlConfig: OWMLConfig }>>("save_owml_config"),
    updateMod: $<ModAction>("update_mod"),
    updateAll: $<ActionCommand<{ uniqueNames: string[] }>>("update_all_mods"),
    runGame: $<EmptyCommand>("run_game"),
    clearLogs: $<ActionCommand<{ port: number }>>("clear_logs"),
    stopLogging: $<ActionCommand<{ port: number }>>("stop_logging"),
    getLogLines: $<CommandInfo<{ port: number, filterType?: number | undefined }, number[]>>("get_log_lines"),
    exportMods: $<ActionCommand<{ path: string }>>("export_mods"),
    importMods: $<ActionCommand<{ path: string }>>("import_mods")
};

type Command = keyof typeof commandInfo;

const makeInvoke = (key: Command) => {
    const name = commandInfo[key];
    return (payload?: (typeof name)[0]) =>
        invoke(name as unknown as string, payload ?? {}) as Promise<(typeof name)[1]>;
};

const makeHook = (key: Command) => {
    const name = commandInfo[key];
    return (eventName: string, payload?: (typeof name)[0]) => {
        const fn = makeInvoke(key);
        return useTauri<(typeof name)[1]>(
            eventName,
            () => fn(payload ?? {}) as unknown as Promise<(typeof name)[1]>,
            payload
        );
    };
};

export type Commands = {
    [T in Command]: (payload?: (typeof commandInfo)[T][0]) => Promise<(typeof commandInfo)[T][1]>;
};

export type Hooks = {
    [T in Command]: (
        eventName: string | string[],
        payload?: (typeof commandInfo)[T][0]
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
