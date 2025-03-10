use clack_host::{
    bundle::PluginBundle,
    factory::PluginFactory,
    host::{AudioProcessorHandler, HostHandlers, HostInfo, MainThreadHandler, SharedHandler},
    plugin::{PluginInstance, PluginInstanceError},
    process::PluginAudioConfiguration,
};

#[derive(Debug, thiserror::Error)]
pub enum ClapInfoHostError {
    #[error("Failed to instantiate plugin")]
    PluginInstanceError(PluginInstanceError),

    #[error("Invalid plugin index: {0}")]
    InvalidPluginIndex(u32),
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

    pub fn activate_plugin(&mut self, index: u32) -> Result<(), ClapInfoHostError> {
        let factory = self.bundle.get_factory::<PluginFactory<'_>>().unwrap();
        let plugin_id = factory
            .plugin_descriptor(index)
            .ok_or(ClapInfoHostError::InvalidPluginIndex(index))?
            .id()
            .expect("Failed to get plugin id");
        let host_info = HostInfo::new(
            "clap-info-rs",
            "danigb",
            "github.com/danigb/clap-info-rs",
            "0.1.0",
        )
        .expect("Static strings should not fail");
        let mut plugin: PluginInstance<Self> = PluginInstance::new(
            |_| ClapInfoSharedHandler::default(),
            |sh: &ClapInfoSharedHandler| ClapInfoMainThreadHandler { sh },
            &self.bundle,
            plugin_id,
            &host_info,
        )?;

        let audio_config = PluginAudioConfiguration {
            sample_rate: 48_000.0,
            min_frames_count: 0,
            max_frames_count: 512,
        };
        let activated =
            plugin.activate(|sh, _| ClipInfoAudioProcessor { sh: &sh }, audio_config)?;
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
