use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::{error, warn, Level};
use owmods_core::{
    db::LocalDatabase,
    mods::local::UnsafeLocalMod,
    progress::{
        ProgressAction, ProgressFinishPayload, ProgressIncrementPayload, ProgressMessagePayload,
        ProgressPayload, ProgressStartPayload, ProgressType,
    },
    validate::ModValidationError,
};

const PROGRESS_TEMPLATE: &str = "{spinner} {wide_msg} [{bar:100.green/cyan}]";
const PROGRESS_CHARS: &str = "=>-";
const SPINNER_TEMPLATE: &str = "{spinner} {msg} {elapsed}";

pub struct Logger {
    multi: MultiProgress,
    bars: Arc<Mutex<HashMap<String, ProgressBar>>>,
}

impl Logger {
    fn start_progress(&self, payload: ProgressStartPayload) {
        let pb = ProgressBar::hidden();
        pb.set_length(payload.len.into());
        let template = if matches!(payload.progress_type, ProgressType::Definite) {
            PROGRESS_TEMPLATE
        } else {
            SPINNER_TEMPLATE
        };
        let style = ProgressStyle::with_template(template)
            .unwrap()
            .progress_chars(PROGRESS_CHARS);
        pb.set_style(style);
        pb.set_message(payload.msg.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        // Try to make downloads and extracts a bit organized
        let pb = match payload.progress_action {
            ProgressAction::Download => self.multi.insert(0, pb),
            ProgressAction::Extract => self.multi.insert_from_back(0, pb),
        };
        self.bars.lock().unwrap().insert(payload.id, pb);
    }

    fn increment_progress(&self, payload: ProgressIncrementPayload) {
        self.bars
            .lock()
            .unwrap()
            .get::<String>(&payload.id)
            .unwrap()
            .set_position(payload.progress.into());
    }

    fn set_message(&self, payload: ProgressMessagePayload) {
        self.bars
            .lock()
            .unwrap()
            .get::<String>(&payload.id)
            .unwrap()
            .set_message(payload.msg.to_string());
    }

    fn finish(&self, payload: ProgressFinishPayload) {
        let bars = self.bars.lock().unwrap();
        let pb = bars.get::<String>(&payload.id).unwrap();
        pb.set_message(payload.msg.to_string());
        if !payload.success {
            pb.set_position(pb.length().unwrap_or(1));
        }
        pb.finish();
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            multi: MultiProgress::default(),
            bars: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::STATIC_MAX_LEVEL
    }

    fn log(&self, record: &log::Record) {
        if record.target() == "progress" {
            let raw = format!("{}", record.args());
            let payload = ProgressPayload::parse(&raw);

            match payload {
                Ok(payload) => match payload {
                    ProgressPayload::Start(payload) => self.start_progress(payload),
                    ProgressPayload::Increment(payload) => self.increment_progress(payload),
                    ProgressPayload::Msg(payload) => self.set_message(payload),
                    ProgressPayload::Finish(payload) => self.finish(payload),
                },
                Err(why) => warn!("Failed to parse progress payload: {why}"),
            };
        } else if self.enabled(record.metadata()) && record.target().starts_with("owmods") {
            let args = format!("{}", record.args());
            let msg = match record.level() {
                Level::Error => args.red(),
                Level::Warn => args.yellow(),
                Level::Info => args.cyan(),
                Level::Debug => args.bright_black(),
                Level::Trace => args.bright_black(),
            };
            println!("{msg}");
        }
    }

    fn flush(&self) {
        todo!()
    }
}

pub fn log_mod_validation_errors(local_mod: &UnsafeLocalMod, local_db: &LocalDatabase) {
    let name: &str = local_mod.get_name();
    for err in local_mod.get_errs() {
        match err {
            ModValidationError::MissingDLL(path) => match path {
                Some(path) => {
                    warn!(
                        "The DLL specified in {name}'s manifest.json ({path}) appears to be missing"
                    )
                }
                None => {
                    warn!("{name} has no DLL specified")
                }
            },
            ModValidationError::DisabledDep(unique_name) => {
                let dep_name = local_db
                    .get_mod(unique_name)
                    .map(|m| &m.manifest.name)
                    .unwrap_or(unique_name);
                error!(
                    "{name} requires {dep_name}, but it's disabled! (run \"owmods check --fix-deps\" to auto-fix)"
                );
            }
            ModValidationError::MissingDep(unique_name) => {
                error!(
                    "{name} requires {unique_name}, but it's missing! (run \"owmods check --fix-deps\" to auto-fix)"
                );
            }
            ModValidationError::ConflictingMod(unique_name) => {
                let conflict_name = local_db
                    .get_mod(unique_name)
                    .map(|m| &m.manifest.name)
                    .unwrap_or(unique_name);
                warn!("{name} conflicts with {conflict_name}!");
            }
            ModValidationError::InvalidManifest(why) => {
                error!("Could not load manifest for {name}: {why}");
            }
            ModValidationError::DuplicateMod(other_path) => {
                error!(
                    "Mod at {name} was already loaded from {other_path}, this may indicate duplicate mods"
                );
            }
            ModValidationError::Outdated(new_version) => {
                error!(
                    "{name} is outdated, consider updating it (latest version is v{new_version})"
                )
            }
        }
    }
}

pub fn show_pre_patcher_warning(mod_name: &str) {
    warn!("========\n{mod_name} possibly modified game files.\nIn order to disable it completely, use the \"verify game files\" option in Steam / Epic.\nCheck {mod_name}'s readme for more information.\n========");
}
