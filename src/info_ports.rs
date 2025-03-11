use clack_extensions::audio_ports::{
    AudioPortFlags, AudioPortInfo, AudioPortInfoBuffer, AudioPortType, PluginAudioPorts,
};
use clack_extensions::audio_ports_config::{
    AudioPortsConfigBuffer, AudioPortsConfiguration, PluginAudioPortsConfig,
};
use clack_extensions::note_ports::{
    NoteDialect, NoteDialects, NotePortInfo, NotePortInfoBuffer, PluginNotePorts,
};
use clack_host::plugin::PluginMainThreadHandle;
use serde::Serialize;
use std::ffi::CStr;

#[derive(Serialize)]
pub struct InfoAudioPorts {
    implemented: bool,
    input_port_count: u32,
    output_port_count: u32,
    input_ports: Vec<InfoAudioPort>,
    output_ports: Vec<InfoAudioPort>,
}

#[derive(Serialize)]
pub struct InfoAudioPort {
    id: u32,
    name: String,
    port_type: String,
    channel_count: u32,
    flags: InfoAudioPortFlag,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_place_pair: Option<u32>,
}

impl InfoAudioPort {
    pub fn from_port_info(port_info: &AudioPortInfo) -> Self {
        Self {
            id: port_info.id.into(),
            name: String::from_utf8_lossy(port_info.name).to_string(),
            port_type: Self::port_type_to_string(port_info.port_type),
            channel_count: port_info.channel_count,
            flags: InfoAudioPortFlag {
                fields: Self::port_flags_to_str_list(port_info.flags),
                value: port_info.flags.bits(),
            },
            in_place_pair: port_info.in_place_pair.map(Into::into),
        }
    }

    fn port_type_to_string(port_type: Option<AudioPortType>) -> String {
        if let Some(pt) = port_type {
            if pt == AudioPortType::MONO {
                "mono".to_string()
            } else if pt == AudioPortType::STEREO {
                "stereo".to_string()
            } else {
                "unknown".to_string()
            }
        } else {
            "unknown".to_string()
        }
    }
    fn port_flags_to_str_list(flags: AudioPortFlags) -> Option<Vec<&'static str>> {
        let mut flag_fields = Vec::new();
        if flags.contains(AudioPortFlags::IS_MAIN) {
            flag_fields.push("CLAP_AUDIO_PORT_IS_MAIN");
        }
        if flags.contains(AudioPortFlags::SUPPORTS_64BITS) {
            flag_fields.push("CLAP_AUDIO_PORT_SUPPORTS_64BITS");
        }
        if flags.contains(AudioPortFlags::REQUIRES_COMMON_SAMPLE_SIZE) {
            flag_fields.push("CLAP_AUDIO_PORT_REQUIRES_COMMON_SAMPLE_SIZE");
        }
        if flags.contains(AudioPortFlags::PREFERS_64BITS) {
            flag_fields.push("CLAP_AUDIO_PORT_PREFERS_64BITS");
        }

        if flag_fields.is_empty() {
            None
        } else {
            Some(flag_fields)
        }
    }
}

#[derive(Serialize)]
pub struct InfoAudioPortFlag {
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<&'static str>>,
    value: u32,
}

impl InfoAudioPorts {
    pub fn from_plugin(plugin: &mut PluginMainThreadHandle) -> Self {
        let mut input_ports = Vec::new();
        let mut output_ports = Vec::new();
        let mut implemented = false;
        let mut input_port_count = 0;
        let mut output_port_count = 0;

        if let Some(audio_ports) = plugin.get_extension::<PluginAudioPorts>() {
            implemented = true;

            // Get input port count
            input_port_count = audio_ports.count(plugin, true);
            output_port_count = audio_ports.count(plugin, false);

            // Get input ports
            for i in 0..input_port_count {
                let mut buffer = AudioPortInfoBuffer::default();
                if let Some(port_info) = audio_ports.get(plugin, i, true, &mut buffer) {
                    input_ports.push(InfoAudioPort::from_port_info(&port_info));
                }
            }

            // Get output ports
            for i in 0..output_port_count {
                let mut buffer = AudioPortInfoBuffer::default();
                if let Some(port_info) = audio_ports.get(plugin, i, false, &mut buffer) {
                    output_ports.push(InfoAudioPort::from_port_info(&port_info));
                }
            }
        }

        Self {
            implemented,
            input_port_count,
            output_port_count,
            input_ports,
            output_ports,
        }
    }
}

#[derive(Serialize)]
pub struct InfoAudioPortsConfigs {
    implemented: bool,
    count: u32,
    configs: Vec<InfoAudioPortsConfig>,
}

#[derive(Serialize)]
pub struct InfoAudioPortsConfig {
    id: u32,
    name: String,
    input_port_count: u32,
    output_port_count: u32,
    has_main_input: bool,
    has_main_output: bool,
    main_input_channel_count: u32,
    main_input_port_type: String,
    main_output_channel_count: u32,
    main_output_port_type: String,
}

impl InfoAudioPortsConfigs {
    pub fn from_plugin(plugin: &mut PluginMainThreadHandle) -> Self {
        let mut implemented = false;
        let mut count = 0;
        let mut configs = Vec::new();

        if let Some(audio_ports_config) = plugin.get_extension::<PluginAudioPortsConfig>() {
            implemented = true;
            count = audio_ports_config.count(plugin) as u32;

            for i in 0..count {
                let mut buffer = AudioPortsConfigBuffer::default();
                if let Some(config) = audio_ports_config.get(plugin, i as usize, &mut buffer) {
                    // Create configuration from data
                    let name = String::from_utf8_lossy(config.name).to_string();

                    let (has_main_input, main_input_channel_count, main_input_port_type) =
                        if let Some(main_input) = &config.main_input {
                            let port_type =
                                InfoAudioPort::port_type_to_string(main_input.port_type);
                            (true, main_input.channel_count, port_type)
                        } else {
                            (false, 0, "none".to_string())
                        };

                    let (has_main_output, main_output_channel_count, main_output_port_type) =
                        if let Some(main_output) = &config.main_output {
                            let port_type =
                                InfoAudioPort::port_type_to_string(main_output.port_type);
                            (true, main_output.channel_count, port_type)
                        } else {
                            (false, 0, "none".to_string())
                        };

                    configs.push(InfoAudioPortsConfig {
                        id: config.id.into(),
                        name,
                        input_port_count: config.input_port_count,
                        output_port_count: config.output_port_count,
                        has_main_input,
                        has_main_output,
                        main_input_channel_count,
                        main_input_port_type,
                        main_output_channel_count,
                        main_output_port_type,
                    });
                }
            }
        }

        Self {
            implemented,
            count,
            configs,
        }
    }
}

#[derive(Serialize)]
pub struct InfoNotePorts {
    implemented: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_ports: Option<Vec<InfoNotePort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_ports: Option<Vec<InfoNotePort>>,
}

#[derive(Serialize)]
pub struct InfoNotePort {
    id: u32,
    name: String,
    supported_dialects: InfoNoteDialects,
    preferred_dialect: InfoNoteDialect,
}

#[derive(Serialize)]
pub struct InfoNoteDialects {
    supported: Vec<String>,
}

#[derive(Serialize)]
pub struct InfoNoteDialect {
    dialect: String,
}

impl InfoNotePorts {
    pub fn from_plugin(plugin: &mut PluginMainThreadHandle) -> Self {
        let mut implemented = false;
        let mut input_count = None;
        let mut output_count = None;
        let mut input_ports = None;
        let mut output_ports = None;

        if let Some(note_ports) = plugin.get_extension::<PluginNotePorts>() {
            implemented = true;

            // Get input and output counts
            let in_count = note_ports.count(plugin, true);
            let out_count = note_ports.count(plugin, false);

            if in_count > 0 {
                input_count = Some(in_count);
                let mut in_ports = Vec::with_capacity(in_count as usize);

                for i in 0..in_count {
                    let mut buffer = NotePortInfoBuffer::default();
                    if let Some(port_info) = note_ports.get(plugin, i, true, &mut buffer) {
                        // Create NotePort object with info from the query result
                        let name = String::from_utf8_lossy(&port_info.name).to_string();

                        // Get supported dialects
                        let mut supported = Vec::new();
                        if port_info.supported_dialects.contains(NoteDialects::CLAP) {
                            supported.push("clap".to_string());
                        }
                        if port_info.supported_dialects.contains(NoteDialects::MIDI) {
                            supported.push("midi".to_string());
                        }
                        if port_info
                            .supported_dialects
                            .contains(NoteDialects::MIDI_MPE)
                        {
                            supported.push("midi-mpe".to_string());
                        }
                        if port_info.supported_dialects.contains(NoteDialects::MIDI2) {
                            supported.push("midi2".to_string());
                        }

                        // Convert preferred dialect to string
                        let dialect = match port_info.preferred_dialect {
                            Some(dialect) => format!("{:?}", dialect).to_lowercase(),
                            None => "unknown".to_string(),
                        };

                        in_ports.push(InfoNotePort {
                            id: port_info.id.into(),
                            name,
                            supported_dialects: InfoNoteDialects { supported },
                            preferred_dialect: InfoNoteDialect { dialect },
                        });
                    }
                }

                if !in_ports.is_empty() {
                    input_ports = Some(in_ports);
                }
            }

            if out_count > 0 {
                output_count = Some(out_count);
                let mut out_ports = Vec::with_capacity(out_count as usize);

                for i in 0..out_count {
                    let mut buffer = NotePortInfoBuffer::default();
                    if let Some(port_info) = note_ports.get(plugin, i, false, &mut buffer) {
                        // Create NotePort object with info from the query result
                        let name = String::from_utf8_lossy(&port_info.name).to_string();

                        // Get supported dialects
                        let mut supported = Vec::new();
                        if port_info.supported_dialects.contains(NoteDialects::CLAP) {
                            supported.push("clap".to_string());
                        }
                        if port_info.supported_dialects.contains(NoteDialects::MIDI) {
                            supported.push("midi".to_string());
                        }
                        if port_info
                            .supported_dialects
                            .contains(NoteDialects::MIDI_MPE)
                        {
                            supported.push("midi-mpe".to_string());
                        }
                        if port_info.supported_dialects.contains(NoteDialects::MIDI2) {
                            supported.push("midi2".to_string());
                        }

                        // Convert preferred dialect to string
                        let dialect = match port_info.preferred_dialect {
                            Some(dialect) => format!("{:?}", dialect).to_lowercase(),
                            None => "unknown".to_string(),
                        };

                        out_ports.push(InfoNotePort {
                            id: port_info.id.into(),
                            name,
                            supported_dialects: InfoNoteDialects { supported },
                            preferred_dialect: InfoNoteDialect { dialect },
                        });
                    }
                }

                if !out_ports.is_empty() {
                    output_ports = Some(out_ports);
                }
            }
        }

        Self {
            implemented,
            input_count,
            output_count,
            input_ports,
            output_ports,
        }
    }
}

#[derive(serde::Serialize)]
pub struct AudioPortsConfigEntry {
    id: String,
    name: String,
    #[serde(rename = "input-port-count")]
    input_port_count: u32,
    #[serde(rename = "output-port-count")]
    output_port_count: u32,
    #[serde(rename = "has-main-input")]
    has_main_input: bool,
    #[serde(rename = "main-input-channel-count")]
    main_input_channel_count: u32,
    #[serde(rename = "main-input-port_type")]
    main_input_port_type: String,
    #[serde(rename = "has-main-output")]
    has_main_output: bool,
    #[serde(rename = "main-output-channel-count")]
    main_output_channel_count: u32,
    #[serde(rename = "main-output-port_type")]
    main_output_port_type: String,
}

#[derive(serde::Serialize)]
pub struct InfoAudioPortsConfigExtension {
    implemented: bool,
    count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    configs: Option<Vec<AudioPortsConfigEntry>>,
}

impl InfoAudioPortsConfigExtension {
    pub fn from_plugin(plugin: &mut PluginMainThreadHandle) -> Self {
        let extension = plugin.get_extension::<PluginAudioPortsConfig>();
        let mut implemented = false;
        let mut count = 0;
        let mut configs = None;

        if let Some(extension) = extension {
            implemented = true;
            count = extension.count(plugin);

            if count > 0 {
                let mut config_entries = Vec::new();
                for i in 0..count {
                    let mut buffer = AudioPortsConfigBuffer::default();
                    if let Some(config) = extension.get(plugin, i, &mut buffer) {
                        // Convert ID to string
                        let id = config.id.to_string();

                        // Convert name from byte array to string
                        let name = String::from_utf8_lossy(&config.name).to_string();

                        // Handle main input
                        let (has_main_input, main_input_channel_count, main_input_port_type) =
                            if let Some(main_input) = &config.main_input {
                                let port_type =
                                    InfoAudioPort::port_type_to_string(main_input.port_type);
                                (true, main_input.channel_count, port_type)
                            } else {
                                (false, 0, "none".to_string())
                            };

                        // Handle main output
                        let (has_main_output, main_output_channel_count, main_output_port_type) =
                            if let Some(main_output) = &config.main_output {
                                let port_type =
                                    InfoAudioPort::port_type_to_string(main_output.port_type);
                                (true, main_output.channel_count, port_type)
                            } else {
                                (false, 0, "none".to_string())
                            };

                        config_entries.push(AudioPortsConfigEntry {
                            id,
                            name,
                            input_port_count: config.input_port_count,
                            output_port_count: config.output_port_count,
                            has_main_input,
                            main_input_channel_count,
                            main_input_port_type,
                            has_main_output,
                            main_output_channel_count,
                            main_output_port_type,
                        });
                    }
                }

                if !config_entries.is_empty() {
                    configs = Some(config_entries);
                }
            }
        }

        Self {
            implemented,
            count,
            configs,
        }
    }
}
