use clap::Parser;
use clap_scanner::ClapScanner;

mod clap_scanner;

#[derive(Parser)]
struct ClapInfoArgs {
    #[arg(short, long)]
    list_clap_files: bool,
    #[arg(short, long)]
    scan_clap_files: bool,
}

#[derive(serde::Serialize)]
struct ClapInfoResult<T: ?Sized + serde::Serialize> {
    action: &'static str,
    result: T,
}

fn main() {
    let args = ClapInfoArgs::parse();

    if args.list_clap_files {
        let clap_files = ClapScanner::installed_claps()
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>();

        let result = ClapInfoResult {
            action: "display paths for installed claps",
            result: clap_files,
        };

        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else if args.scan_clap_files {
        let clap_bundles = ClapScanner::installed_claps()
            .iter()
            .filter_map(|p| ClapScanner::get_bundle_info(p))
            .collect::<Vec<_>>();

        let result = ClapInfoResult {
            action: "display descriptions for installed claps",
            result: clap_bundles,
        };

        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    }
}
