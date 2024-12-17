use anyhow::Result;
use log::debug;
use owmods_core::protocol::{ProtocolPayload, ProtocolVerb};
use tauri::{async_runtime, Manager};

use crate::events::{CustomEventEmitterAll, Event};

/// Amount of listeners that need to be active before we can emit the protocol invoke event.
pub const PROTOCOL_LISTENER_AMOUNT: usize = 2;

#[cfg(not(target_os = "linux"))]
fn register_or_listen<F>(uri: &str, handler: F) -> Result<()>
where
    F: FnMut(String) + Send + 'static,
{
    debug!("Registering protocol handler for {}", uri);
    tauri_plugin_deep_link::register(uri, handler)?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn register_or_listen<F>(_: &str, handler: F) -> Result<()>
where
    F: FnMut(String) + Send + 'static,
{
    debug!("Skipping protocol registration for Linux");
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
                if cfg!(target_os = "macos") {
                    // MacOS doesn't use cmd line arguments for protocol invoke like Windows and Linux
                    // Meaning opposed to being able to be read like in main.rs, we have to ensure
                    // that the listeners are ready to receive the protocol payload and store it
                    // otherwise.
                    let handle2 = handle.clone();
                    async_runtime::spawn(async move {
                        let state = handle2.state::<crate::State>();
                        let listeners = state.protocol_listeners.read().await;
                        if listeners.len() < PROTOCOL_LISTENER_AMOUNT {
                            let mut payload = state.protocol_url.write().await;
                            *payload = Some(protocol_payload);
                        } else {
                            handle2
                                .typed_emit_all(&Event::ProtocolInvoke(protocol_payload))
                                .ok();
                        }
                    });
                } else {
                    handle
                        .typed_emit_all(&Event::ProtocolInvoke(protocol_payload))
                        .ok();
                }
            }
        }
    })
}
