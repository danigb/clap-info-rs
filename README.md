# clap-info-rs

> A simple command line CLAP information tool in Rust

⚠️ Disclaimer: This is a Rust port of [clap-info](https://github.com/free-audio/clap-info/) made for the solely purpose of learning [CLAP](https://github.com/free-audio/clap), [clap-sys](https://github.com/micahrj/clap-sys) and [clack](https://github.com/prokopyl/clack) all together. Although I've tried the output to be as similar as possible as `clap-info`, the goal of this project is to learn Rust clap development, not to replace that tool.

## Install and build

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

- [-] Implement `--search-path`

  - [x] Find all possible CLAP locations for all platforms
  - [ ] Read environment variables

- [x] Implement `--list-clap-files`

  - [x] Find all .clap packages inside folders

- [x] Implement `--scan-clap-files`

  - [x] Read bundle information of all clap plugins

- [-] Implement plugin info

  - [x] Read params
  - [-] Read ports
  - [ ] Scan other extensions
