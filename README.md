# clap-info-rs

> A simple command line CLAP information tool in Rust

⚠️ Disclaimer: This is a Rust port of [clap-info](https://github.com/free-audio/clap-info/) made for the solely purpose of learning [CLAP](https://github.com/free-audio/clap), [clap-sys]() and [clack](https://github.com/prokopyl/clack) t ptogether. The goal of this project is **not to replace `clap-info` tool** but show how to do it with Rust.

## Install and build

Clone repository and build target:

```bash
git clone git@github.com:danigb/clap-info-rs.git
cargo build --release
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

## Development plan

- [-] Implement `--list-clap-files`

  - [-] Find all possible CLAP locations for all platforms
  - [ ] Read environment variables
  - [x] Find all .clap packages inside folders
  - [x] Print output as json

- [x] Implement `--scan-clap-files`

  - [x] Read bundle information of all clap plugins
  - [x] Print output as json

- [ ] Implement plugin info

- [ ] Implement `--search-path`
