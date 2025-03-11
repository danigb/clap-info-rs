# clap-info-rs

> A simple command line CLAP information tool in Rust

⚠️ Disclaimer: This is a Rust port of [clap-info](https://github.com/free-audio/clap-info/) made for the solely purpose of learning [CLAP](https://github.com/free-audio/clap), [clap-sys](https://github.com/micahrj/clap-sys) and [clack](https://github.com/prokopyl/clack) all together. Although I've tried the output to be as similar as possible as `clap-info`, the goal of this project is to learn Rust clap development, not to replace that tool.

`clap-info-rs` is still a work in progress. Some features are not yet implemented.

## Install and usage

Clone repository and build target:

```bash
git clone git@github.com:danigb/clap-info-rs.git
cargo build --release
```

Run the built target:

```bash
./target/release/clap-info-rs -l
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

## License

MIT License
