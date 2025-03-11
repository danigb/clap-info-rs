use clack_host::{
    bundle::PluginBundle,
    factory::PluginFactory,
    host::{AudioProcessorHandler, HostHandlers, HostInfo, MainThreadHandler, SharedHandler},
    plugin::{PluginInstance, PluginInstanceError},
    process::PluginAudioConfiguration,
};

use crate::{InfoParams, InfoPlugin};

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

        // Remove parameter processing for now

        let _audio_config = PluginAudioConfiguration {
            sample_rate: 48_000.0,
            min_frames_count: 32,
            max_frames_count: 4096,
        };

        let mut mt_handle = plugin.plugin_handle();

        // Add params extension info
        let params_info = InfoParams::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.params", params_info);

        // Add audio ports extension info
        let audio_ports = crate::info_ports::InfoAudioPorts::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.audio-ports", audio_ports);

        // Add audio ports config extension info
        let audio_ports_config =
            crate::info_ports::InfoAudioPortsConfigs::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.audio-ports-config", audio_ports_config);

        // Add note ports extension info
        let note_ports = crate::info_ports::InfoNotePorts::from_plugin(&mut mt_handle);
        plugin_info.add_extension("clap.note-ports", note_ports);

        Ok(())
    }
}

impl HostHandlers for ClapInfoHost {
    type AudioProcessor<'a> = ClipInfoAudioProcessor<'a>;
    type Shared<'a> = ClapInfoSharedHandler;
    type MainThread<'a> = ClapInfoMainThreadHandler<'a>;
}

// AudioProcessorHandler
#[derive(Debug, Clone)]
pub struct ClipInfoAudioProcessor<'a> {
    pub sh: &'a ClapInfoSharedHandler,
}
impl<'a> AudioProcessorHandler<'a> for ClipInfoAudioProcessor<'a> {}

// CLAP SharedHandler
#[derive(Debug, Clone, Default)]
pub struct ClapInfoSharedHandler {}

impl SharedHandler<'_> for ClapInfoSharedHandler {
    fn request_restart(&self) {}
    fn request_process(&self) {}
    fn request_callback(&self) {}
}

// CLAP MainThreadHandler
#[derive(Debug)]
pub struct ClapInfoMainThreadHandler<'a> {
    pub sh: &'a ClapInfoSharedHandler,
}

impl<'a> MainThreadHandler<'a> for ClapInfoMainThreadHandler<'a> {}
