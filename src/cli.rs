//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use std::io;
use std::path::{Path, PathBuf};

use anyhow::{ensure, Context, Result};
use structopt::clap::{crate_name, AppSettings, Shell};
use structopt::StructOpt;
use strum::VariantNames;

use crate::config::Config;
use crate::long_version;
use crate::value::{Color, Format, InputFormat, OutputFormat};

#[derive(StructOpt)]
#[structopt(
    long_version = long_version!().as_str(),
    about,
    after_help = "See dsconv(1) for more details.",
    settings = &[AppSettings::ColoredHelp, AppSettings::DeriveDisplayOrder]
)]
pub struct Opt {
    /// Specify input format.
    ///
    /// This option can be omitted if the input file is specified and <FORMAT>
    /// can be determined from the filename extension.
    #[structopt(
        short,
        long,
        value_name = "FORMAT",
        possible_values = &InputFormat::VARIANTS,
        case_insensitive = true
    )]
    pub from: Option<Format>,

    /// Specify output format.
    ///
    /// This option can be omitted if the output file is specified and <FORMAT>
    /// can be determined from the filename extension.
    #[structopt(
        short,
        long,
        value_name = "FORMAT",
        possible_values = &OutputFormat::VARIANTS,
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
    #[structopt(short, long, value_name = "FILE", conflicts_with = "color")]
    pub output: Option<PathBuf>,

    /// Output as a pretty-printed string.
    ///
    /// If the value is omitted, it is the same as selecting `true`.
    /// The value of this option is case-sensitive.
    /// This option is available when the output is JSON or TOML.
    #[structopt(short, long, value_name = "BOOLEAN", possible_values = &["true", "false"])]
    pub pretty: Option<Option<bool>>,

    /// Specify when to use colored output.
    #[structopt(
        long,
        value_name = "WHEN",
        possible_values = &Color::VARIANTS,
        case_insensitive = true,
        default_value
    )]
    pub color: Color,

    /// Input from <FILE>.
    #[structopt(value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Generate shell completion.
    ///
    /// The generated shell completion is output to stdout.
    /// To output as a shell completion file, specify the directory to store
    /// using `--output`=<OUT_DIR>.
    #[structopt(long, value_name = "SHELL", possible_values = &Shell::variants())]
    pub generate_completion: Option<Shell>,
}

impl Opt {
    /// Apply the config from the config file.
    pub fn apply_config(mut self) -> Result<Self> {
        if let Some(path) = Config::path() {
            let config = Config::read(path)?;

            if let Some(pretty) = config.pretty {
                if self.pretty.is_none() {
                    self.pretty = Some(Some(pretty));
                }
            }
        }

        Ok(self)
    }

    /// Generate shell completion to stdout.
    pub fn generate_completion(shell: Shell) {
        Self::clap().gen_completions_to(crate_name!(), shell, &mut io::stdout())
    }

    /// Generate shell completion to a file.
    pub fn generate_completion_to(shell: Shell, out_dir: impl AsRef<Path>) -> Result<()> {
        let out_dir = out_dir
            .as_ref()
            .canonicalize()
            .context("Failed to generate shell completion to a file")?;
        ensure!(out_dir.is_dir(), "Output destination is not a directory");

        Self::clap().gen_completions(crate_name!(), shell, &out_dir);
        eprintln!(
            "Generated a shell completion file of the {} in {}",
            shell,
            out_dir.display()
        );

        Ok(())
    }
}
