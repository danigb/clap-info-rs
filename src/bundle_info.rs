use std::{collections::HashMap, ffi::CStr, path::PathBuf};

use clack_host::{bundle::PluginBundle, factory::PluginFactory};

#[derive(Debug, serde::Serialize)]
pub struct PluginInfo {
    id: String,
    name: String,
    description: String,
    vendor: String,
    version: String,
    features: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extensions: Option<HashMap<String, String>>,
}

#[derive(Debug, serde::Serialize)]
pub struct BundleInfo {
    clap_version: String,
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bundle_file: Option<String>,
    plugins: Vec<PluginInfo>,
}

impl BundleInfo {
    pub fn new(path: String, bundle: &PluginBundle, bundle_file: Option<PathBuf>) -> Self {
        let factory = bundle.get_factory::<PluginFactory<'_>>().unwrap();
        let clap_version = format!("{}", bundle.version());
        let bundle_file = bundle_file.map(|path| path.display().to_string());
        // Convert Option<&CStr> to a String
        let safe_string = |opt: Option<&'_ CStr>| {
            opt.map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default()
        };

        let plugins = factory
            .plugin_descriptors()
            .map(|plugin| PluginInfo {
                id: safe_string(plugin.id()),
                name: safe_string(plugin.name()),
                vendor: safe_string(plugin.vendor()),
                description: safe_string(plugin.description()),
                version: safe_string(plugin.version()),
                features: plugin
                    .features()
                    .map(|f| f.to_string_lossy().to_string())
                    .collect(),
                extensions: None,
            })
            .collect();

        Self {
            clap_version,
            path,
            bundle_file,
            plugins,
        }
    }
}
