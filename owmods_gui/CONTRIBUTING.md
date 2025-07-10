# Contributing

To build this project you'll need rust, cargo, and node/npm.

This package is called `owmods_gui` so anytime you want to perform cargo commands on it **do not do it in this folder**, do it from the root of the repo and add `-p owmods_gui` to your cargo command.

Ex: `cargo add tokio` should become `cargo add tokio -p owmods_gui`.

## Setup on Linux

Please follow the [tauri docs](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux) for instructions on installing the necessary system packages.

### On Nix

Run the `shell.nix` file. Allow insecure is needed for OpenSSL 1.1.1 support.

```sh
NIX_ALLOW_INSECURE=1 nix-shell shell.nix --impure
```

## npm

The frontend for this package is made with TS so you need to install related dependencies. First cd in to `owmods_gui/frontend`, then run `npm i`

## Typeshare

Upon editing any structs marked with `#[typeshare]`, you'll need to regenerate TypeScript bindings.

To do this, you need to install the typeshare cli:

```sh
cargo install typeshare-cli
```

Then run the `gen-types` npm command:

```sh
cd owmods_gui/frontend
npm run gen-types
```

This will generate `types.d.ts` in `owmods_gui/frontend/src/types.d.ts`.

## Formatting & Linting

Please format and lint your code before pushing:

```sh
cargo fmt
cargo lint
```

And lint and format the frontend as well:

```sh
cd owmods_gui/frontend
npm run lint
npm run prettify
```

Git hooks are setup to run clippy on every commit, meaning they may take longer.

## Connection Refused Error When Using Protocol Installs

This is a result of your OS falsely thinking the dev version of the manager should handle protocols, causing it to open a window with only the frontend of the tauri application, which will fail. To remedy this simply open a release version of the manager and it will re-register as the handler for the owmods URI.

## Guides

### Events

Events are used to communicate between the frontend and backend. They are defined in `owmods_gui/src/events.rs`.
The events are then synced via typeshare to the frontend, where they are defined in `owmods_gui/frontend/src/types.d.ts`.

To use an event, you must first define it in `owmods_gui/src/events.rs` and then sync it to the frontend by running `npm gen-types` in `owmods_gui/frontend`.

Then, you can use the event in the frontend by importing `emit` or `listen` from `owmods_gui/frontend/events.ts` and using it like so:

```ts
import { emit } from "./events.ts";

emit("event-name", { data: "some data" });
```

In the backend you can use typed variants of the normal AppHandle methods to emit events. Just make sure you have the `CustomEvent*` traits in scope.
(They're located in `owmods_gui/src/events.rs`)

```rs
use owmods_gui::events::{Event, CustomEventEmitter};

#[tauri::command]
async fn emit_event(app: AppHandle) {
    app.typed_emit(Event::EventName("payload".to_string()));
}
```

### Commands

Commands are used to call Rust functions from the frontend. They are defined in `owmods_gui/src/commands.rs`.
Sadly, they are not synced to the frontend, so you must define them in the frontend as well.
You can define commands in the backend with the `#[tauri::command]` attribute.

```rs
use tauri::AppHandle;

#[tauri::command]
async fn my_command(my_name: String, app: AppHandle) {
    // do something
}
```

Then, add the command to the generate_handler! in `owmods_gui/src/main.rs`.

Now, edit `owmods_gui/frontend/src/commands.ts` and add the command to the `Commands` enum.

```ts
const commands = {
  // ...
  myCommand: $<CommandInfo<{ myName: string }, number>>("my_command"), // the name of the command must match the name in the backend
  // ...
};
```

`CommandInfo` is a generic type that takes the type of the command's arguments and the type of the command's return value.

Some other convenience types are provided, check above the commands object.

Also note how `my_name` became `myName` in the frontend. This is because the frontend uses camelCase, while the backend uses snake_case. Tauri changes the case automatically.

Now you can use the commands in two ways:

```ts
commands.myCommand({ myName: "some name" }).then((result) => {
  // do something with the result
});

// or, in a React component

const [status, result] = hooks.myCommand("someEvent", { myName: "some name" });
```

Hooks is special, the first argument passed will make the hook automatically rerun the command when the event is emitted. If you don't want this behavior, pass `"none"` as the first argument.

It also returns a tuple, where the first element is the status of the command, and the second element is the result of the command. Result can be null so make sure to be type-safe.

If an error occurs in the command:

- commands.\* will show a message dialog with the error (pass false as a second argument to disable this)
- hooks.\* will throw it to the nearest error boundary
