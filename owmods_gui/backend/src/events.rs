use anyhow::{anyhow, Result};
use owmods_core::protocol::ProtocolPayload;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Window};
use typeshare::typeshare;

use crate::{game::GameMessage, LogPort};

pub const INVOKE_URI: &str = "owmods://events/invoke";

fn map_emit_err(e: tauri::Error) -> anyhow::Error {
    anyhow!("Error Emitting Event: {e:?}")
}

#[typeshare]
pub type EmptyParams = ();

// Typeshare doesn't allow tuples, I cry
#[typeshare]
#[derive(Deserialize, Serialize, Clone)]
pub struct LogLineCountUpdatePayload {
    pub port: LogPort,
    pub line: u32,
}

#[typeshare]
#[derive(Deserialize, Serialize, Clone)]
pub struct LogsBehindPayload {
    pub port: LogPort,
    pub behind: bool,
}

#[typeshare]
#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "name", content = "params", rename_all = "camelCase")]
pub enum Event {
    LocalRefresh(EmptyParams),
    RemoteRefresh(EmptyParams),
    RemoteInitialized(EmptyParams),
    ModBusy(EmptyParams),
    ConfigReload(EmptyParams),
    GuiConfigReload(bool),
    OwmlConfigReload(EmptyParams),
    GameStart(LogPort),
    LogUpdate(LogPort),
    LogLineCountUpdate(LogLineCountUpdatePayload),
    LogFatal(GameMessage),
    LogsBehind(LogsBehindPayload),
    ProtocolInvoke(ProtocolPayload),
    ProgressUpdate(EmptyParams),
    ProgressBatchFinish(bool),
    DragEnter(EmptyParams),
    DragLeave(EmptyParams),
    // Used on frontend
    #[allow(dead_code)]
    OpenOwmlSetup(EmptyParams),
    RequestReload(String),
    /// Purposefully never used, some hooks only need to run once
    #[allow(dead_code)]
    None(EmptyParams),
}

pub trait CustomEventEmitter {
    fn typed_emit(&self, event: &Event) -> Result<()>;
}

pub trait CustomEventTriggerGlobal {
    fn typed_trigger_global(&self, event: &Event) -> Result<()>;
}

pub trait CustomEventEmitterAll {
    fn typed_emit_all(&self, event: &Event) -> Result<()>;
}

pub trait CustomEventListener {
    fn typed_listen<F: Fn(Event) + Send + Sync + 'static>(&self, f: F);
}

impl CustomEventEmitterAll for AppHandle {
    fn typed_emit_all(&self, event: &Event) -> Result<()> {
        self.emit(INVOKE_URI, event).map_err(map_emit_err)
    }
}

// impl CustomEventTriggerGlobal for AppHandle {
//     fn typed_trigger_global(&self, event: &Event) -> Result<()> {
//         self.trigger(INVOKE_URI, Some(serde_json::to_string(event).unwrap()));
//         Ok(())
//     }
// }

impl CustomEventListener for AppHandle {
    fn typed_listen<F: Fn(Event) + Send + Sync + 'static>(&self, f: F) {
        self.clone()
            .listen_global(INVOKE_URI, move |e: tauri::Event| {
                let event = e
                    .payload();
                let event = serde_json::from_str::<Event>(event).ok();
                if let Some(event) = event {
                    f(event);
                }
            });
    }
}

impl CustomEventEmitter for Window {
    fn typed_emit(&self, event: &Event) -> Result<()> {
        self.emit(INVOKE_URI, event).map_err(map_emit_err)
    }
}

impl CustomEventEmitterAll for Window {
    fn typed_emit_all(&self, event: &Event) -> Result<()> {
        self.emit(INVOKE_URI, event).map_err(map_emit_err)
    }
}
