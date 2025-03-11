# clap-info-rs

> A simple command line CLAP information tool in Rust

⚠️ Disclaimer: This is a Rust port of [clap-info](https://github.com/free-audio/clap-info/) made for the solely purpose of learning [CLAP](https://github.com/free-audio/clap), [clap-sys](https://github.com/micahrj/clap-sys) and [clack](https://github.com/prokopyl/clack) all together. Although I've tried the output to be as similar as possible as `clap-info`, the goal of this project is to learn Rust clap development, not to replace that tool.

## Install and build

Clone repository and build target:

```bash
git clone git@github.com:danigb/clap-info-rs.git
cargo build --release
```

Or run with cargo:

```bash
$ cargo run

A tool to display information about CLAP plugins

Usage: clap-info-rs [OPTIONS] [PATH]

Arguments:
  [PATH]  The path to the CLAP plugin to display information about

Options:
  -l, --list-clap-files  Show all CLAP files in the search path then exit
  -s, --scan-clap-files  Show all descriptions in all CLAP files in the search path, then exit
      --search-path      Show the CLAP plugin search paths then exit
  -w, --which <WHICH>    Choose which plugin to create (if the CLAP has more than one).
  -h, --help             Print help
```

## Usage

List (all installed clap files):

```bash
./target/release/clap-info-rs -l
```

Scan (show bundle information of installed clap files):

```bash
./target/release/clap-info-rs -s
```

Info (show information of an installed plugin):

```bash
./target/release/clap-info-rs /Library/Audio/Plug-Ins/CLAP/Airwindows Consolidated.clap
```

## License

MIT License
