use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use clap_info_rs::{BundleInfo, ClapInfoHost, ClapScanner};

#[derive(Parser)]
#[command(about = "A tool to display information about CLAP plugins")]
struct ClapInfoArgs {
    /// The path to the CLAP plugin to display information about
    path: Option<String>,

    /// List all installed CLAP plugins
    #[arg(short = 'l', long)]
    list_clap_files: bool,

    /// List descriptions for all installed CLAP plugins
    #[arg(short = 's', long)]
    scan_clap_files: bool,

    /// The index of the plugin to display information about
    #[arg(short, long, default_value = "0")]
    plugin_index: usize,
}

#[derive(serde::Serialize)]
struct ClapInfoResult<T: ?Sized + serde::Serialize> {
    action: &'static str,
    result: T,
}

fn main() {
    let args = ClapInfoArgs::parse();

    if let Some(ref path) = args.path {
        match ClapScanner::get_bundle(PathBuf::from(path)) {
            Some((bundle, file)) => {
                let mut info = BundleInfo::new(path.to_owned(), &bundle, Some(file));
                let mut host = ClapInfoHost::new(bundle);
                let mut plugin_info = info.get_plugin_mut(args.plugin_index);
                host.query_extensions(args.plugin_index, &mut plugin_info)
                    .unwrap();

                let result = ClapInfoResult {
                    action: "display info for a CLAP plugin",
                    result: info,
                };
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            }
            None => {
                eprintln!("Failed to get bundle info for {}", args.path.unwrap());
            }
        }
    } else if args.list_clap_files {
        let clap_files = ClapScanner::installed_claps()
            .into_iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>();

        let result = ClapInfoResult {
            action: "display paths for installed claps",
            result: clap_files,
        };

        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else if args.scan_clap_files {
        let clap_bundles = ClapScanner::installed_claps()
            .into_iter()
            .filter_map(|clap_path| {
                let path = clap_path.display().to_string();
                ClapScanner::get_bundle(clap_path)
                    .map(|(bundle, bundle_path)| BundleInfo::new(path, &bundle, Some(bundle_path)))
            })
            .collect::<Vec<_>>();

        let result = ClapInfoResult {
            action: "display descriptions for installed claps",
            result: clap_bundles,
        };

        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        println!("{}", ClapInfoArgs::command().render_help());
    }
}
