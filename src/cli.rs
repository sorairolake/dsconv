//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use std::io;
use std::path::{Path, PathBuf};

use anyhow::{ensure, Context, Result};
use const_format::formatcp;
use structopt::clap::{crate_name, crate_version, AppSettings, Shell};
use structopt::StructOpt;

use crate::config::Config;
use crate::value::Format;

const COMMIT_HASH: &str = if let Some(hash) = option_env!("VERGEN_GIT_SHA") {
    hash
} else {
    ""
};
const COMMIT_DATE: &str = if let Some(date) = option_env!("VERGEN_GIT_COMMIT_DATE") {
    date
} else {
    ""
};
const LONG_VERSION: &str = if !COMMIT_HASH.is_empty() && !COMMIT_DATE.is_empty() {
    formatcp!(
        "{} (built for {})\n\n{}\n{}\nCommit hash: {}\nCommit date: {}\n{}",
        crate_version!(),
        env!("VERGEN_CARGO_TARGET_TRIPLE"),
        "Copyright (C) 2021 Shun Sakai",
        "License: Apache License 2.0",
        COMMIT_HASH,
        COMMIT_DATE,
        "Reporting bugs: https://github.com/sorairolake/dsconv/issues"
    )
} else {
    formatcp!(
        "{} (built for {})\n\n{}\n{}\n{}",
        crate_version!(),
        env!("VERGEN_CARGO_TARGET_TRIPLE"),
        "Copyright (C) 2021 Shun Sakai",
        "License: Apache License 2.0",
        "Reporting bugs: https://github.com/sorairolake/dsconv/issues"
    )
};
const APP_SETTINGS: [AppSettings; 2] = [AppSettings::ColoredHelp, AppSettings::DeriveDisplayOrder];
const INPUT_FORMATS: [&str; 8] = [
    "cbor",
    "hjson",
    "json",
    "json5",
    "messagepack",
    "ron",
    "toml",
    "yaml",
];
const OUTPUT_FORMATS: [&str; 5] = ["cbor", "json", "messagepack", "toml", "yaml"];

#[derive(StructOpt)]
#[structopt(long_version = LONG_VERSION, about, settings = &APP_SETTINGS)]
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
    #[structopt(short, long, value_name = "BOOLEAN", possible_values = &["true", "false"])]
    pub pretty: Option<Option<bool>>,

    /// Input from <FILE>.
    #[structopt(value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Generate shell completion.
    #[structopt(long, value_name = "SHELL", possible_values = &Shell::variants(), hidden = true)]
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
