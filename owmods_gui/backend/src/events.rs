use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Window};
use typeshare::typeshare;

use crate::{game::GameMessage, protocol::ProtocolPayload, LogPort};

const INVOKE_URI: &str = "owmods://events/invoke";

fn map_emit_err(e: tauri::Error) -> anyhow::Error {
    anyhow!("Error Emitting Event: {e:?}")
}

#[typeshare]
pub type EmptyParams = ();

#[typeshare]
#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "name", content = "params", rename_all = "camelCase")]
pub enum Event {
    LocalRefresh(EmptyParams),
    RemoteRefresh(EmptyParams),
    ModBusy(EmptyParams),
    ConfigReload(EmptyParams),
    GuiConfigReload(bool),
    OwmlConfigReload(EmptyParams),
    GameStart(LogPort),
    LogUpdate(LogPort),
    LogFatal(GameMessage),
    ProtocolInvoke(ProtocolPayload),
    ProgressUpdate(EmptyParams),
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

impl CustomEventEmitterAll for AppHandle {
    fn typed_emit_all(&self, event: &Event) -> Result<()> {
        self.emit_all(INVOKE_URI, event).map_err(map_emit_err)
    }
}

impl CustomEventTriggerGlobal for AppHandle {
    fn typed_trigger_global(&self, event: &Event) -> Result<()> {
        self.trigger_global(INVOKE_URI, Some(serde_json::to_string(event).unwrap()));
        Ok(())
    }
}

impl CustomEventEmitter for Window {
    fn typed_emit(&self, event: &Event) -> Result<()> {
        self.emit(INVOKE_URI, event).map_err(map_emit_err)
    }
}

impl CustomEventEmitterAll for Window {
    fn typed_emit_all(&self, event: &Event) -> Result<()> {
        self.emit_all(INVOKE_URI, event).map_err(map_emit_err)
    }
}
