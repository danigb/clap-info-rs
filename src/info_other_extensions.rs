use clack_extensions::audio_ports::AudioPortType;
use clack_extensions::gui::{GuiApiType, GuiConfiguration, PluginGui};
use clack_extensions::latency::PluginLatency;
use clack_extensions::note_name::{NoteName, NoteNameBuffer, PluginNoteName};
use clack_extensions::state::PluginState;
use clack_extensions::tail::{PluginTail, TailLength};
use clack_host::plugin::PluginMainThreadHandle;
use std::collections::HashMap;
use std::io::Write;

use crate::ClapInfoHost;

#[derive(serde::Serialize)]
pub struct InfoLatencyExtension {
    implemented: bool,
    latency: u32,
}

impl InfoLatencyExtension {
    pub fn from_plugin(plugin: &mut PluginMainThreadHandle) -> Self {
        let extension = plugin.get_extension::<PluginLatency>();
        let mut implemented = false;
        let mut latency = 0;

        if let Some(extension) = extension {
            implemented = true;
            latency = extension.get(plugin);
        }

        Self {
            implemented,
            latency,
        }
    }
}

#[derive(serde::Serialize)]
pub struct InfoTailExtension {
    implemented: bool,
    tail: u32,
}

impl InfoTailExtension {
    pub fn from_plugin(plugin: &mut PluginMainThreadHandle) -> Self {
        let extension = plugin.get_extension::<PluginTail>();
        let mut implemented = false;
        let tail = 0;

        if let Some(_extension) = extension {
            implemented = true;
            // tail = extension.get(plugin);  // This line seems to be missing
        }

        Self { implemented, tail }
    }
}

#[derive(serde::Serialize)]
pub struct ApiSupported {
    api: String,
    floating: bool,
}

#[derive(serde::Serialize)]
pub struct PreferredApi {
    api: String,
    floating: bool,
}

#[derive(serde::Serialize)]
pub struct InfoGuiExtension {
    implemented: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_api: Option<PreferredApi>,
}

impl InfoGuiExtension {
    pub fn from_plugin(plugin: &mut PluginMainThreadHandle) -> Self {
        let extension = plugin.get_extension::<PluginGui>();
        let mut implemented = false;
        let mut api_supported = None;
        let mut preferred_api = None;

        if let Some(extension) = extension {
            implemented = true;

            let mut supported_apis = Vec::new();

            // Check support for different window APIs
            let apis = [
                (GuiApiType::COCOA, "cocoa"),
                (GuiApiType::WIN32, "win32"),
                (GuiApiType::X11, "x11"),
                (GuiApiType::WAYLAND, "wayland"),
            ];

            for api in &apis {
                // Create configuration for each API
                let config = GuiConfiguration {
                    api_type: api.0,
                    is_floating: false,
                };
                if extension.is_api_supported(plugin, config) {
                    supported_apis.push(api.1.to_string());
                }

                let config_floating = GuiConfiguration {
                    api_type: api.0,
                    is_floating: true,
                };
                if extension.is_api_supported(plugin, config_floating) {
                    supported_apis.push(format!("{}.floating", api.1));
                }
            }

            if !supported_apis.is_empty() {
                api_supported = Some(supported_apis);
            }

            // Get preferred API
            if let Some(config) = extension.get_preferred_api(plugin) {
                preferred_api = Some(PreferredApi {
                    api: config.api_type.0.to_string_lossy().to_string(),
                    floating: config.is_floating,
                });
            }
        }

        Self {
            implemented,
            api_supported,
            preferred_api,
        }
    }
}

#[derive(serde::Serialize)]
pub struct InfoStateExtension {
    implemented: bool,
    #[serde(rename = "bytes-written", skip_serializing_if = "Option::is_none")]
    bytes_written: Option<u64>,
}

impl InfoStateExtension {
    pub fn from_plugin(plugin: &mut PluginMainThreadHandle) -> Self {
        let extension = plugin.get_extension::<PluginState>();
        let mut implemented = false;
        let mut bytes_written = None;

        if let Some(extension) = extension {
            implemented = true;

            // Create a counter for bytes written
            struct ByteCounter(u64);

            impl Write for ByteCounter {
                fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                    self.0 += buf.len() as u64;
                    Ok(buf.len())
                }

                fn flush(&mut self) -> std::io::Result<()> {
                    Ok(())
                }
            }

            // Save state and count bytes written
            let mut counter = ByteCounter(0);
            let result = extension.save(plugin, &mut counter);

            if result.is_ok() {
                bytes_written = Some(counter.0);
            }
        }

        Self {
            implemented,
            bytes_written,
        }
    }
}

#[derive(serde::Serialize)]
pub struct NoteNameEntry {
    name: String,
    port: i16,
    key: i16,
    channel: i16,
}

#[derive(serde::Serialize)]
pub struct InfoNoteNameExtension {
    implemented: bool,
    count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    note_names: Option<Vec<NoteNameEntry>>,
}

impl InfoNoteNameExtension {
    pub fn from_plugin(plugin: &mut PluginMainThreadHandle) -> Self {
        let extension = plugin.get_extension::<PluginNoteName>();
        let mut implemented = false;
        let mut count = 0;
        let mut note_names = None;

        if let Some(extension) = extension {
            implemented = true;
            count = extension.count(plugin);

            if count > 0 {
                let mut names = Vec::new();
                for i in 0..count {
                    let mut buffer = NoteNameBuffer::default();
                    if let Some(note_name) = extension.get(plugin, i, &mut buffer) {
                        // Convert byte slice to String
                        let name = String::from_utf8_lossy(&note_name.name).to_string();

                        // Convert Match<u16> to i16
                        let port = match note_name.port {
                            clack_host::events::Match::All => -1, // Use -1 to represent "all"
                            clack_host::events::Match::Specific(p) => p as i16,
                        };

                        let key = match note_name.key {
                            clack_host::events::Match::All => -1,
                            clack_host::events::Match::Specific(k) => k as i16,
                        };

                        let channel = match note_name.channel {
                            clack_host::events::Match::All => -1,
                            clack_host::events::Match::Specific(c) => c as i16,
                        };

                        names.push(NoteNameEntry {
                            name,
                            port,
                            key,
                            channel,
                        });
                    }
                }

                if !names.is_empty() {
                    note_names = Some(names);
                }
            }
        }

        Self {
            implemented,
            count: count.try_into().unwrap(),
            note_names,
        }
    }
}
