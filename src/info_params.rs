use clack_extensions::params::{ParamInfo, ParamInfoBuffer, ParamInfoFlags, PluginParams};
use clack_host::plugin::PluginMainThreadHandle;

#[derive(serde::Serialize)]
pub struct InfoParams {
    implemented: bool,
    param_count: u32,
    params: Vec<InfoParam>,
}

impl InfoParams {
    pub fn from_plugin(plugin: &mut PluginMainThreadHandle) -> Self {
        let mut param_list = Vec::new();
        let mut implemented = false;
        let mut param_count = 0;

        if let Some(params) = plugin.get_extension::<PluginParams>() {
            implemented = true;
            param_count = params.count(plugin);

            let mut buffer = ParamInfoBuffer::new();
            for i in 0..param_count {
                let param_info = params.get_info(plugin, i, &mut buffer);
                if let Some(param_info) = param_info {
                    param_list.push(InfoParam::from_param_info(&param_info));
                }
            }
        }
        return Self {
            implemented,
            param_count,
            params: param_list,
        };
    }
}

#[derive(serde::Serialize)]
pub struct InfoParam {
    id: String,
    name: String,
    flags: Vec<&'static str>,
    values: InfoParamValue,
}

impl InfoParam {
    pub fn from_param_info(param_info: &ParamInfo) -> Self {
        Self {
            id: param_info.id.to_string(),
            name: String::from_utf8_lossy(param_info.name).to_string(),
            flags: Self::flags_to_vec(param_info.flags),
            values: InfoParamValue {
                current: param_info.default_value,
                default: param_info.default_value,
                min: param_info.min_value,
                max: param_info.max_value,
            },
        }
    }

    fn flags_to_vec(flags: ParamInfoFlags) -> Vec<&'static str> {
        let mut result = Vec::new();

        if flags.contains(ParamInfoFlags::IS_STEPPED) {
            result.push("stepped");
        }
        if flags.contains(ParamInfoFlags::IS_PERIODIC) {
            result.push("periodic");
        }
        if flags.contains(ParamInfoFlags::IS_PERIODIC) {
            result.push("periodic");
        }
        if flags.contains(ParamInfoFlags::IS_READONLY) {
            result.push("readonly");
        }
        if flags.contains(ParamInfoFlags::IS_BYPASS) {
            result.push("bypass");
        }

        if flags.contains(ParamInfoFlags::IS_AUTOMATABLE) {
            result.push("auto");
        }
        if flags.contains(ParamInfoFlags::IS_AUTOMATABLE_PER_CHANNEL) {
            result.push("auto-per-channel");
        }
        if flags.contains(ParamInfoFlags::IS_AUTOMATABLE_PER_KEY) {
            result.push("auto-per-key");
        }
        if flags.contains(ParamInfoFlags::IS_AUTOMATABLE_PER_NOTE_ID) {
            result.push("auto-per-note-id");
        }
        if flags.contains(ParamInfoFlags::IS_AUTOMATABLE_PER_PORT) {
            result.push("auto-per-port");
        }

        if flags.contains(ParamInfoFlags::IS_MODULATABLE) {
            result.push("mod");
        }
        if flags.contains(ParamInfoFlags::IS_MODULATABLE_PER_CHANNEL) {
            result.push("mod-per-channel");
        }
        if flags.contains(ParamInfoFlags::IS_MODULATABLE_PER_KEY) {
            result.push("mod-per-key");
        }
        if flags.contains(ParamInfoFlags::IS_MODULATABLE_PER_NOTE_ID) {
            result.push("mod-per-note-id");
        }
        if flags.contains(ParamInfoFlags::IS_MODULATABLE_PER_PORT) {
            result.push("mod-per-port");
        }
        if flags.contains(ParamInfoFlags::REQUIRES_PROCESS) {
            result.push("requires-process");
        }

        result
    }
}

#[derive(serde::Serialize)]
pub struct InfoParamValue {
    current: f64,
    default: f64,
    min: f64,
    max: f64,
}
