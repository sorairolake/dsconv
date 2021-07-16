//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{ensure, Context, Result};
use const_format::formatcp;
use structopt::clap::{crate_name, crate_version, AppSettings, Shell};
use structopt::StructOpt;

use crate::config::Config;
use crate::value::Format;

const LONG_VERSION: &str = formatcp!(
    "{}\n\n{}\n{}\n{}",
    crate_version!(),
    "Copyright (C) 2021 Shun Sakai",
    "License: Apache License 2.0",
    "Reporting bugs: https://github.com/sorairolake/dsconv/issues"
);
const INPUT_FORMATS: [&str; 7] = [
    "cbor",
    "hjson",
    "json",
    "json5",
    "messagepack",
    "toml",
    "yaml",
];
const OUTPUT_FORMATS: [&str; 5] = ["cbor", "json", "messagepack", "toml", "yaml"];

#[derive(Debug, StructOpt)]
#[structopt(long_version = LONG_VERSION, about, setting = AppSettings::ColoredHelp)]
pub struct Opt {
    /// Specify input format.
    #[structopt(
        short,
        long,
        value_name = "FORMAT",
        possible_values = &INPUT_FORMATS,
        case_insensitive = true
    )]
    pub from: Option<Format>,

    /// Specify output format.
    #[structopt(
        short,
        long,
        value_name = "FORMAT",
        possible_values = &OUTPUT_FORMATS,
        case_insensitive = true
    )]
    pub to: Option<Format>,

    /// List supported input formats.
    #[structopt(long, conflicts_with = "list-output-formats")]
    pub list_input_formats: bool,

    /// List supported output formats.
    #[structopt(long)]
    pub list_output_formats: bool,

    /// Output to <FILE> instead of stdout.
    #[structopt(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Output as a pretty-printed string.
    #[structopt(long, value_name = "BOOLEAN", possible_values = &["true", "false"])]
    pub pretty: Option<Option<bool>>,

    /// Input from <FILE>.
    #[structopt(value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Generate shell completion.
    #[structopt(long, value_name = "SHELL", possible_values = &Shell::variants(), hidden = true)]
    pub generate_completion: Option<Shell>,
}

impl Opt {
    /// Guess the format from the extension.
    fn guess_format(ext: &str) -> Option<Format> {
        match ext {
            "hjson" => Some(Format::Hjson),
            "json" => Some(Format::Json),
            "json5" => Some(Format::Json5),
            "toml" => Some(Format::Toml),
            "yaml" | "yml" => Some(Format::Yaml),
            _ => None,
        }
    }

    /// Guess the input format from the extension of a input file.
    pub fn guess_input_format(mut self) -> Self {
        if self.from.is_some() {
            return self;
        }

        if let Some(ref f) = self.input {
            self.from = f
                .extension()
                .and_then(OsStr::to_str)
                .and_then(Self::guess_format);
        }

        self
    }

    /// Guess the output format from the extension of a output file.
    pub fn guess_output_format(mut self) -> Self {
        if self.to.is_some() {
            return self;
        }

        if let Some(ref f) = self.output {
            self.to = f
                .extension()
                .and_then(OsStr::to_str)
                .and_then(Self::guess_format);
        }

        self
    }

    /// Apply the config from the config file.
    pub fn apply_config(mut self) -> Result<Self> {
        if let Some(p) = Config::path() {
            let config = Config::read(p)?;

            if let Some(p) = config.pretty {
                if self.pretty.is_none() {
                    self.pretty = Some(Some(p));
                }
            }
        }

        Ok(self)
    }

    /// Generate shell completion to a file.
    pub fn generate_completion_to_file(shell: Shell, outdir: impl AsRef<Path>) -> Result<()> {
        let outdir = outdir
            .as_ref()
            .canonicalize()
            .context("Failed to generate shell completion to a file")?;
        ensure!(outdir.is_dir(), "Output destination is not a directory");

        Self::clap().gen_completions(crate_name!(), shell, &outdir);
        eprintln!(
            "Generated a shell completion file of the {} in {}",
            shell,
            outdir.display()
        );

        Ok(())
    }

    /// Generate shell completion to stdout.
    pub fn generate_completion_to_stdout(shell: Shell) {
        Self::clap().gen_completions_to(crate_name!(), shell, &mut io::stdout())
    }
}
