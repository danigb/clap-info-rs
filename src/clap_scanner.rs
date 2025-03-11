use std::{collections::HashMap, ffi::CStr, path::PathBuf};

use clack_host::{bundle::PluginBundle, factory::PluginFactory};

use crate::InfoBundle;

pub struct ClapScanner;

impl ClapScanner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn installed_claps() -> Vec<PathBuf> {
        let search_paths = Self::get_search_paths();
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

    // Try to get a bundle from a bundle path (potentially a directory with multiple files inside).
    pub fn get_bundle(path: PathBuf) -> Option<(PluginBundle, PathBuf)> {
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
                .map(|bundle| (bundle, path))
        } else {
            None
        }
    }

    pub fn get_search_paths() -> Vec<String> {
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
