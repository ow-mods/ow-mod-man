# owmods-core

The core library for the Outer Wilds mod manager, this package is responsible for basically everything from fetching the db to downloading mods.  
The other two packages in this repo act as frontends to this package.

## Usage

Before getting started your program needs to handle two things:

1. Implementing a LoggerBackend and ProgressHandler and creating a Logger
2. Loading the config

These will need to be passed to nearly any function in the library

### Implementing Traits

Implement `LoggerBackend` and `ProgressHandler`, these will handle outputting data about what the library is doing while it's working.

Then, create a new logger `owmods_core::logging::Logger::new(Box::new(myLoggerBackend));`.

### Loading Config

Call `owmods_core::config::get_config(&logger);`

### Errors

Nearly every function returns `Result<something, anyhow::Error>`, so make sure to handle those.
I'd recommend forwarding errors to the logger by doing:

```rs
use owmods_core::log;

log!(logger, error, "{:?}", theError);
```

### On Linux

On Linux the wine_prefix config variable must be set before running `owmods_core::game::launch_game`.

To do this call `owmods_core::game::setup_wine_prefix`. **Note that while although this method exists for windows builds, you should not call it (it'll just log an error but still).**

It's recommended to only setup the prefix if the wine_prefix setting is `None`, also if you intend for user to use your program you may want to explain what's happening.

## Modules

- alerts.rs: Manages fetching and showing alerts
- config.rs: Manages the manager's configuration
- db.rs: Manages fetching and compiling the local and remote database
- download.rs: Manages downloading and installing mods and OWML
- game.rs: Manages running the game and setting up a wine prefix on linux
- io.rs: Manages importing and exporting the list of mods
- lib.rs: Exports stuff
- logging.rs: Creates traits that dependents are expected to implement for logging
- mods.rs: Hols the structs that represent local and remote mods and some utility functions
- open.rs: Manages opening shortcuts and READMEs
- remove.rs: Manages uninstalling mods
- toggle.rs: Manages toggling mods on and off, and checking if they're enabled
- updates.rs: Manages updating mods and OWML
- utils.rs (private): Some utility functions used internally
- validate.rs: Manages checking for and fixing conflicts and missing dependencies

## Vocab

**Local Mod**: A mod that is installed locally  
**Remote Mod**: A mod that's in the database  
**Local DB**: A compilation of all local mods  
**Remote DB**: A compilation of all remote mods  
