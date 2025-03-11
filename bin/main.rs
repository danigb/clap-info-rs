use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use clap_info_rs::{ClapInfoHost, ClapScanner, InfoBundle};

#[derive(Parser)]
#[command(about = "A tool to display information about CLAP plugins")]
struct ClapInfoArgs {
    /// The path to the CLAP plugin to display information about
    path: Option<String>,

    /// Show all CLAP files in the search path then exit
    #[arg(short = 'l', long)]
    list_clap_files: bool,

    /// Show all descriptions in all CLAP files in the search path, then exit
    #[arg(short = 's', long)]
    scan_clap_files: bool,

    /// Show the CLAP plugin search paths then exit
    #[arg(long)]
    search_path: bool,

    // TODO: If you set to -1 we will traverse all plugins.
    /// Choose which plugin to create (if the CLAP has more than one).
    #[arg(short, long, default_value = "0")]
    which: usize,
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
                let mut info = InfoBundle::new(path.to_owned(), &bundle, Some(file));
                let mut host = ClapInfoHost::new(bundle);
                let mut plugin_info = info.get_plugin_mut(args.which);
                host.query_extensions(args.which, &mut plugin_info).unwrap();

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
    } else if args.search_path {
        let search_path = ClapScanner::get_search_paths();
        let result = ClapInfoResult {
            action: "display the CLAP plugin search path",
            result: search_path,
        };
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
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
                    .map(|(bundle, bundle_path)| InfoBundle::new(path, &bundle, Some(bundle_path)))
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
