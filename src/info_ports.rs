use clack_extensions::{
    audio_ports::{AudioPortInfo, PluginAudioPorts},
    note_ports::PluginNotePorts,
};
use clack_host::plugin::PluginMainThreadHandle;

#[derive(serde::Serialize)]
pub struct InfoAudioPorts {
    implemented: bool,
    input_port_count: u32,
    output_port_count: u32,
    input_ports: Vec<InfoAudioPort>,
    output_ports: Vec<InfoAudioPort>,
}

#[derive(serde::Serialize)]
pub struct InfoAudioPort {
    id: u32,
    name: String,
    port_type: String,
    channel_count: u32,
    flags: Vec<String>,
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

pub struct InfoNotePorts {
    implemented: bool,
}

impl InfoNotePorts {
    pub fn from_plugin(plugin: &mut PluginMainThreadHandle) -> Self {
        let mut implemented = false;

        if let Some(note_ports) = plugin.get_extension::<PluginNotePorts>() {
            implemented = true;
        }

        Self { implemented }
    }
}
