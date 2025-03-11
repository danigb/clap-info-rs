use std::{collections::HashMap, ffi::CStr, path::PathBuf};

use clack_host::{
    bundle::PluginBundle,
    factory::{PluginDescriptor, PluginFactory},
};

#[derive(serde::Serialize)]
#[serde(rename_all = "kebab-case")]
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
        Self {
            clap_version: format!("{}", bundle.version()),
            path,
            bundle_file: bundle_file.map(|path| path.display().to_string()),
            plugins: factory
                .plugin_descriptors()
                .map(|descriptor| InfoPlugin::from_descriptor(&descriptor))
                .collect(),
        }
    }

    pub fn get_plugin_mut(&mut self, index: usize) -> &mut InfoPlugin {
        &mut self.plugins[index]
    }
}

pub struct InfoPlugin {
    descriptor: InfoPluginDescriptor,
    extensions: Option<HashMap<String, serde_json::Value>>,
}

impl InfoPlugin {
    pub fn from_descriptor(descriptor: &PluginDescriptor<'_>) -> Self {
        Self {
            descriptor: InfoPluginDescriptor::from_descriptor(descriptor),
            extensions: None,
        }
    }

    pub fn add_extension<T: serde::Serialize>(&mut self, key: &str, value: T) {
        self.extensions
            .get_or_insert(HashMap::new())
            .insert(key.to_string(), serde_json::to_value(value).unwrap());
    }
}

// Custom serialization implementation to handle the case where extensions is None
// We do this to mimic original clap-info tool: it returns a different json
// whenever it's scanning all bundles or a single one.
impl serde::Serialize for InfoPlugin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.extensions.is_none() || self.extensions.as_ref().unwrap().is_empty() {
            // If no extensions, just serialize the descriptor
            self.descriptor.serialize(serializer)
        } else {
            // Otherwise, serialize both descriptor and extensions
            use serde::ser::SerializeStruct;
            let mut state = serializer.serialize_struct("InfoPlugin", 2)?;
            state.serialize_field("descriptor", &self.descriptor)?;
            state.serialize_field("extensions", &self.extensions)?;
            state.end()
        }
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InfoPluginDescriptor {
    id: String,
    name: String,
    description: String,
    vendor: String,
    version: String,
    features: Vec<String>,
}

impl InfoPluginDescriptor {
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
        }
    }
}
