import { LoadState, useTauri } from "@hooks";
import { invoke } from "@tauri-apps/api";
import { Config, GuiConfig, LocalMod, OWMLConfig, RemoteMod } from "@types";

type CommandInfo<P, R> = [P, R];
type GetCommand<V> = CommandInfo<Record<string, never>, V>;
type EmptyCommand = GetCommand<void>;
type ActionCommand<P> = CommandInfo<P, void>;
type ModCommand<M> = CommandInfo<{ uniqueName: string }, M>;
type ModAction = ModCommand<void>;

const $ = <T>() => null as T;

const commandInfo = {
    refresh_local_db: $<EmptyCommand>(),
    refresh_remote_db: $<EmptyCommand>(),
    fetch_config: $<GetCommand<Config>>(),
    get_gui_config: $<GetCommand<GuiConfig>>(),
    get_owml_config: $<GetCommand<OWMLConfig>>(),
    get_local_mods: $<GetCommand<string[]>>(),
    get_remote_mods: $<CommandInfo<{ filter: string }, string[]>>(),
    get_updatable_mods: $<GetCommand<string[]>>(),
    get_local_mod: $<ModCommand<LocalMod>>(),
    get_remote_mod: $<ModCommand<RemoteMod>>(),
    toggle_mod: $<ActionCommand<{ uniqueName: string; enabled: boolean }>>(),
    open_mod_folder: $<ModAction>(),
    open_mod_readme: $<ModAction>(),
    uninstall_mod: $<ModAction>(),
    install_mod: $<ModAction>(),
    install_url: $<ActionCommand<{ url: string }>>(),
    install_zip: $<ActionCommand<{ path: string }>>(),
    install_owml: $<EmptyCommand>(),
    set_owml: $<CommandInfo<{ path: string }, boolean>>(),
    save_config: $<ActionCommand<{ config: Config }>>(),
    save_gui_config: $<ActionCommand<{ guiConfig: GuiConfig }>>(),
    save_owml_config: $<ActionCommand<{ owmlConfig: OWMLConfig }>>(),
    update_mod: $<ModAction>(),
    update_all_mods: $<ActionCommand<{ uniqueNames: string[] }>>()
};

type Command = keyof typeof commandInfo;

const makeInvoke = (key: Command) => {
    const tmpObj = commandInfo[key];
    return (payload?: (typeof tmpObj)[0]) =>
        invoke(key, payload ?? {}) as Promise<(typeof tmpObj)[1]>;
};

const makeHook = (key: Command) => {
    const tmpObj = commandInfo[key];
    return (eventName: string, payload?: (typeof tmpObj)[0]) => {
        const fn = makeInvoke(key);
        return useTauri<(typeof tmpObj)[1]>(
            eventName,
            () => fn(payload ?? {}) as unknown as Promise<(typeof tmpObj)[1]>,
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
