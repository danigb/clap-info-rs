use std::{collections::HashMap, ffi::CStr, path::PathBuf};

use clack_host::{
    bundle::PluginBundle,
    factory::{PluginDescriptor, PluginFactory},
};

#[derive(Debug, serde::Serialize)]
pub struct InfoBundle {
    clap_version: String,
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bundle_file: Option<String>,
    plugins: Vec<InfoPlugin>,
}

impl InfoBundle {
    pub fn new(path: String, bundle: &PluginBundle, bundle_file: Option<PathBuf>) -> Self {
        let factory = bundle.get_factory::<PluginFactory<'_>>().unwrap();
        let clap_version = format!("{}", bundle.version());
        let bundle_file = bundle_file.map(|path| path.display().to_string());

        let plugins = factory
            .plugin_descriptors()
            .map(|descriptor| InfoPlugin::from_descriptor(&descriptor))
            .collect();

        Self {
            clap_version,
            path,
            bundle_file,
            plugins,
        }
    }

    pub fn get_plugin_mut(&mut self, index: usize) -> &mut InfoPlugin {
        &mut self.plugins[index]
    }
}

#[derive(Debug, serde::Serialize)]
pub struct InfoPlugin {
    id: String,
    name: String,
    description: String,
    vendor: String,
    version: String,
    features: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extensions: Option<HashMap<String, serde_json::Value>>,
}

impl InfoPlugin {
    fn from_descriptor(descriptor: &PluginDescriptor<'_>) -> Self {
        // Convert Option<&CStr> to a String
        let safe_string = |opt: Option<&'_ CStr>| {
            opt.map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default()
        };
        Self {
            id: safe_string(descriptor.id()),
            name: safe_string(descriptor.name()),
            vendor: safe_string(descriptor.vendor()),
            description: safe_string(descriptor.description()),
            version: safe_string(descriptor.version()),
            features: descriptor
                .features()
                .map(|f| f.to_string_lossy().to_string())
                .collect(),
            extensions: None,
        }
    }

    pub fn add_extension<T: serde::Serialize>(&mut self, key: &str, value: T) {
        self.extensions
            .get_or_insert(HashMap::new())
            .insert(key.to_string(), serde_json::to_value(value).unwrap());
    }
}
