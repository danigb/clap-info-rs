use std::path::PathBuf;

pub struct ClapScanner {}

impl ClapScanner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn installed_claps() -> Vec<std::path::PathBuf> {
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
