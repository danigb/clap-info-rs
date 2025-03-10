use std::{ffi::CStr, path::PathBuf};

use clack_host::{bundle::PluginBundle, factory::PluginFactory};

#[derive(Debug, serde::Serialize)]
pub struct BundleInfo {
    clap_version: String,
    path: String,
    bundle_file: String,
    plugins: Vec<PluginInfo>,
}

#[derive(Debug, serde::Serialize)]
pub struct PluginInfo {
    id: String,
    name: String,
    description: String,
    vendor: String,
    version: String,
    features: Vec<String>,
}

pub struct ClapScanner {}

impl ClapScanner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn installed_claps() -> Vec<PathBuf> {
        let search_paths = Self::valid_clap_search_paths();
        let mut claps = Vec::new();

        for path in search_paths {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("clap") {
                        if cfg!(target_os = "macos") {
                            if path.is_dir() {
                                claps.push(path);
                            }
                        } else {
                            if !path.is_dir() {
                                claps.push(path);
                            }
                        }
                    }
                }
            }
        }

        claps
    }

    pub fn get_bundle_info(clap_path: &PathBuf) -> Option<BundleInfo> {
        if let Some((file, bundle)) = Self::get_bundle(clap_path.to_owned()) {
            let factory = bundle.get_factory::<PluginFactory<'_>>().unwrap();
            let clap_version = format!("{}", bundle.version());
            let path = clap_path.display().to_string();
            let bundle_file = file.display().to_string();

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
                })
                .collect();
            Some(BundleInfo {
                clap_version,
                path,
                bundle_file,
                plugins,
            })
        } else {
            None
        }
    }

    pub fn get_bundle(path: PathBuf) -> Option<(PathBuf, PluginBundle)> {
        if path.is_dir() {
            match std::fs::read_dir(path) {
                Ok(dir) => dir.into_iter().find_map(|entry| {
                    entry
                        .ok()
                        .map(|entry| Self::get_bundle(entry.path()))
                        .flatten()
                }),

                Err(_) => None,
            }
        } else if path.is_file() {
            unsafe { PluginBundle::load(&path) }
                .ok()
                .map(|bundle| (path, bundle))
        } else {
            None
        }
    }

    fn valid_clap_search_paths() -> Vec<String> {
        #[cfg(target_os = "linux")]
        {
            vec![
                "/usr/lib/clap".to_string(),
                shellexpand::tilde("~/.clap").to_string(),
            ]
        }

        #[cfg(target_os = "macos")]
        {
            vec![
                "/Library/Audio/Plug-Ins/CLAP".to_string(),
                shellexpand::tilde("~/Library/Audio/Plug-Ins/CLAP").to_string(),
            ]
        }
        #[cfg(target_os = "windows")]
        {
            vec![
                "%COMMONPROGRAMFILES%\\CLAP".to_string(),
                "%LOCALAPPDATA%\\Programs\\Common\\CLAP".to_string(),
                shellexpand::tilde("~/Library/Audio/Plug-Ins/CLAP").to_string(),
            ]
        }
    }
}
