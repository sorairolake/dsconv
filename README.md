# dsconv

[![CI](https://github.com/sorairolake/dsconv/workflows/CI/badge.svg)](https://github.com/sorairolake/dsconv/actions?query=workflow%3ACI)
[![Version](https://img.shields.io/crates/v/dsconv)](https://crates.io/crates/dsconv)
[![License](https://img.shields.io/crates/l/dsconv)](https://apache.org/licenses/LICENSE-2.0)

**dsconv** is a command-line utility for converting from one data-serialization format to another.

## Installation

### Via a package manager

| OS  | Method | Package                                     | Command                |
| --- | ------ | ------------------------------------------- | ---------------------- |
| Any | Cargo  | [`dsconv`](https://crates.io/crates/dsconv) | `cargo install dsconv` |

### Via pre-built binaries

Pre-built binaries for Linux, macOS and Windows are available on the [release page](https://github.com/sorairolake/dsconv/releases).

### How to build and install

Please see [BUILD.adoc](BUILD.adoc).

## Usage

```text
dsconv 0.3.0
A data-serialization format converter

USAGE:
    dsconv [OPTIONS] [FILE]

ARGS:
    <FILE>    Input from <FILE>

OPTIONS:
    -f, --from <FORMAT>
            Specify input format [possible values: cbor, hjson, json, json5, messagepack, ron, toml,
            yaml]

    -t, --to <FORMAT>
            Specify output format [possible values: cbor, json, messagepack, toml, yaml]

        --list-input-formats
            List supported input formats

        --list-output-formats
            List supported output formats

    -o, --output <FILE>
            Output to <FILE> instead of stdout

    -p, --pretty <BOOLEAN>
            Output as a pretty-printed string [possible values: true, false]

        --color <WHEN>
            Specify when to use colored output [default: auto] [possible values: auto, always,
            never]

        --generate-completion <SHELL>
            Generate shell completion [possible values: bash, elvish, fish, powershell, zsh]

    -h, --help
            Print help information

    -V, --version
            Print version information

See dsconv(1) for more details.
```

See [`dsconv(1)`](doc/man/man1/dsconv.1.adoc) for more details.

## Changelog

Please see [CHANGELOG.adoc](CHANGELOG.adoc).

## Configuration

If you want to change the default behavior, you can use the configuration file.

See [`dsconv-config.toml(5)`](doc/man/man5/dsconv-config.toml.5.adoc) for more details.

## Contributing

Please see [CONTRIBUTING.adoc](CONTRIBUTING.adoc).

## License

Copyright (C) 2021 Shun Sakai (see [AUTHORS.adoc](AUTHORS.adoc))

This program is distributed under the terms of the _Apache License 2.0_.

See [COPYING](COPYING) for more details.
