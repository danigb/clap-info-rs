use clack_extensions::params::{ParamInfoBuffer, PluginParams};
use clack_host::plugin::PluginMainThreadHandle;

#[derive(serde::Serialize)]
pub struct ClapParams {
    implemented: bool,
    param_count: u32,
    params: Vec<ClapParam>,
}

#[derive(serde::Serialize)]
pub struct ClapParam {
    id: String,
    name: String,
    flags: Vec<String>,
    values: ParamValues,
}

#[derive(serde::Serialize)]
pub struct ParamValues {
    current: f64,
    default: f64,
    min: f64,
    max: f64,
}

impl ClapParams {
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
                    param_list.push(ClapParam {
                        id: param_info.id.to_string(),
                        name: String::from_utf8_lossy(param_info.name).to_string(),
                        flags: Vec::new(),
                        values: ParamValues {
                            current: 0.0,
                            default: 0.0,
                            min: 0.0,
                            max: 0.0,
                        },
                    });
                }
            }
        }
        return ClapParams {
            implemented,
            param_count,
            params: param_list,
        };
    }
}
