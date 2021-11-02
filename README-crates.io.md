# dsconv

![CI](https://github.com/sorairolake/dsconv/workflows/CI/badge.svg)
![Version](https://img.shields.io/crates/v/dsconv)
![License](https://img.shields.io/crates/l/dsconv)

**dsconv** is a command-line utility for converting from one
data-serialization format to another.

## Installation

### Via a package manager

| OS  | Method | Package                                     | Command                |
|-----|--------|---------------------------------------------|------------------------|
| Any | Cargo  | [`dsconv`](https://crates.io/crates/dsconv) | `cargo install dsconv` |

### Via pre-built binaries

Pre-built binaries for Linux, macOS and Windows are available on the
[release page](https://github.com/sorairolake/dsconv/releases).

### How to build and install

Please see [BUILD.adoc](BUILD.adoc).

## Usage

    dsconv 0.2.0
    A data-serialization format converter

    USAGE:
        dsconv [FLAGS] [OPTIONS] [FILE]

    FLAGS:
            --list-input-formats     List supported input formats
            --list-output-formats    List supported output formats
        -h, --help                   Prints help information
        -V, --version                Prints version information

    OPTIONS:
        -f, --from <FORMAT>       Specify input format [possible values: CBOR, Hjson, JSON, JSON5, MessagePack, RON, TOML,
                                  YAML]
        -t, --to <FORMAT>         Specify output format [possible values: CBOR, JSON, MessagePack, TOML, YAML]
        -o, --output <FILE>       Output to <FILE> instead of stdout
        -p, --pretty <BOOLEAN>    Output as a pretty-printed string [possible values: true, false]

    ARGS:
        <FILE>    Input from <FILE>

    See dsconv(1) for more details.

See [`dsconv(1)`](doc/man/man1/dsconv.1.adoc) for more details.

## Changelog

Please see [CHANGELOG.adoc](CHANGELOG.adoc).

## Configuration

If you want to change the default behavior, you can use the
configuration file.

See [`dsconv-config.toml(5)`](doc/man/man5/dsconv-config.toml.5.adoc)
for more details.

## Contributing

Please see [CONTRIBUTING.adoc](CONTRIBUTING.adoc).

## License

Copyright © 2021 Shun Sakai (see [AUTHORS.adoc](AUTHORS.adoc))

This program is distributed under the terms of the *Apache License 2.0*.

See [COPYING](COPYING) for more details.
