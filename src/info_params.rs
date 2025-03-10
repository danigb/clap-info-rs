use clack_extensions::params::{ParamInfo, ParamInfoBuffer, PluginParams};
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
    flags: Vec<String>,
    values: InfoParamValue,
}

impl InfoParam {
    pub fn from_param_info(param_info: &ParamInfo) -> Self {
        Self {
            id: param_info.id.to_string(),
            name: String::from_utf8_lossy(param_info.name).to_string(),
            flags: Vec::new(),
            values: InfoParamValue {
                current: param_info.default_value,
                default: param_info.default_value,
                min: param_info.min_value,
                max: param_info.max_value,
            },
        }
    }
}

#[derive(serde::Serialize)]
pub struct InfoParamValue {
    current: f64,
    default: f64,
    min: f64,
    max: f64,
}
