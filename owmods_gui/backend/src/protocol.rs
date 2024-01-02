use anyhow::Result;
use log::debug;
use owmods_core::protocol::{ProtocolPayload, ProtocolVerb};

use crate::events::{CustomEventEmitterAll, Event};

#[cfg(windows)]
fn register_or_listen<F>(uri: &str, handler: F) -> Result<()>
where
    F: FnMut(String) + Send + 'static,
{
    debug!("Registering protocol handler for {}", uri);
    tauri_plugin_deep_link::register(uri, handler)?;
    Ok(())
}

#[cfg(not(windows))]
fn register_or_listen<F>(_: &str, handler: F) -> Result<()>
where
    F: FnMut(String) + Send + 'static,
{
    debug!("Skipping protocol registration for non-Windows platform");
    tauri_plugin_deep_link::listen(handler)?;
    Ok(())
}

pub fn prep_protocol(handle: tauri::AppHandle) -> Result<()> {
    register_or_listen("owmods", move |request| {
        let protocol_payload = ProtocolPayload::parse(&request);
        match protocol_payload.verb {
            ProtocolVerb::Unknown => {
                debug!("Unknown protocol verb: {}", request);
            }
            _ => {
                debug!(
                    "Invoking {:?} with {} from protocol",
                    protocol_payload.verb, protocol_payload.payload
                );
                handle
                    .typed_emit_all(&Event::ProtocolInvoke(protocol_payload))
                    .ok();
            }
        }
    })
}
