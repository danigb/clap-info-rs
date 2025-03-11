use clack_host::{
    bundle::PluginBundle,
    factory::PluginFactory,
    host::{AudioProcessorHandler, HostHandlers, HostInfo, MainThreadHandler, SharedHandler},
    plugin::{PluginInstance, PluginInstanceError},
    process::PluginAudioConfiguration,
};

use crate::{
    InfoAudioPortsConfigExtension, InfoGuiExtension, InfoLatencyExtension, InfoNoteNameExtension,
    InfoParams, InfoPlugin, InfoStateExtension, InfoTailExtension,
};

#[derive(Debug, thiserror::Error)]
pub enum ClapInfoHostError {
    #[error("Failed to instantiate plugin")]
    PluginInstanceError(PluginInstanceError),

    #[error("Invalid plugin index: {0}")]
    InvalidPluginIndex(usize),
}

impl From<PluginInstanceError> for ClapInfoHostError {
    fn from(value: PluginInstanceError) -> Self {
        ClapInfoHostError::PluginInstanceError(value)
    }
}

// A minimal host implementation that just queries plugin extensions
pub struct ClapInfoHost {
    bundle: PluginBundle,
}

impl ClapInfoHost {
    pub fn new(bundle: PluginBundle) -> Self {
        Self { bundle }
    }

    pub fn query_extensions(
        &mut self,
        index: usize,
        plugin_info: &mut InfoPlugin,
    ) -> Result<(), ClapInfoHostError> {
        let factory = self.bundle.get_factory::<PluginFactory<'_>>().unwrap();
        let plugin_id = factory
            .plugin_descriptor(index as u32)
            .ok_or(ClapInfoHostError::InvalidPluginIndex(index))?
            .id()
            .expect("Failed to get plugin id");

        let host_info = HostInfo::new(
            "clap-info-rs",
            "danigb",
            "github.com/danigb/clap-info-rs",
            "0.1.0",
        )
        .expect("Static &str props never fail");

        let mut plugin: PluginInstance<Self> = PluginInstance::new(
            |_| ClapInfoSharedHandler::default(),
            |sh: &ClapInfoSharedHandler| ClapInfoMainThreadHandler { sh },
            &self.bundle,
            plugin_id,
            &host_info,
        )?;

        let audio_config = PluginAudioConfiguration {
            sample_rate: 48_000.0,
            min_frames_count: 32,
            max_frames_count: 4096,
        };
        // We need to activate the processor to obtain some data (like latency)
        let _audio_processor = plugin.activate(|sh, _| ClipInfoAudioProcessor { sh }, audio_config);

        let mut mt_handle = plugin.plugin_handle();

        let params_info = InfoParams::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.params", params_info);

        let audio_ports = crate::info_ports::InfoAudioPorts::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.audio-ports", audio_ports);

        let audio_ports_config =
            crate::info_ports::InfoAudioPortsConfigs::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.audio-ports-config", audio_ports_config);

        let note_ports = crate::info_ports::InfoNotePorts::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.note-ports", note_ports);

        let latency_extension = InfoLatencyExtension::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.latency", latency_extension);

        let tail_extension = InfoTailExtension::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.tail", tail_extension);

        let gui_extension = InfoGuiExtension::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.gui", gui_extension);

        let state_extension = InfoStateExtension::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.state", state_extension);

        let note_name_extension = InfoNoteNameExtension::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.note-name", note_name_extension);

        let audio_ports_config_extension =
            InfoAudioPortsConfigExtension::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.audio-ports-config", audio_ports_config_extension);

        Ok(())
    }
}

impl HostHandlers for ClapInfoHost {
    type AudioProcessor<'a> = ClipInfoAudioProcessor<'a>;
    type Shared<'a> = ClapInfoSharedHandler;
    type MainThread<'a> = ClapInfoMainThreadHandler<'a>;
}

#[derive(Debug, Clone)]
pub struct ClipInfoAudioProcessor<'a> {
    pub sh: &'a ClapInfoSharedHandler,
}
impl<'a> AudioProcessorHandler<'a> for ClipInfoAudioProcessor<'a> {}

#[derive(Debug, Clone, Default)]
pub struct ClapInfoSharedHandler {}

impl SharedHandler<'_> for ClapInfoSharedHandler {
    fn request_restart(&self) {}
    fn request_process(&self) {}
    fn request_callback(&self) {}
}

#[derive(Debug)]
pub struct ClapInfoMainThreadHandler<'a> {
    pub sh: &'a ClapInfoSharedHandler,
}

impl<'a> MainThreadHandler<'a> for ClapInfoMainThreadHandler<'a> {}
