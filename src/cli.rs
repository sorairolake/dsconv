//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use std::ffi::OsStr;
use std::path::PathBuf;

use const_format::formatcp;
use structopt::clap::crate_version;
use structopt::StructOpt;

use crate::value::Format;

const COPYRIGHT: &str = "Copyright (C) 2021 Shun Sakai";
const LICENSE: &str = "License: Apache License 2.0";
const REPORTING_BUGS: &str = "Reporting bugs: https://github.com/sorairolake/dsconv/issues";
const LONG_VERSION: &str = formatcp!(
    "{}\n\n{}\n{}\n{}",
    crate_version!(),
    COPYRIGHT,
    LICENSE,
    REPORTING_BUGS
);

#[derive(Debug, StructOpt)]
#[structopt(long_version = LONG_VERSION, about)]
pub struct Opt {
    /// Specify input format.
    #[structopt(short, long, value_name = "FORMAT", possible_values = &["JSON", "YAML", "TOML"], case_insensitive = true)]
    pub from: Option<Format>,

    /// Specify output format.
    #[structopt(short, long, value_name = "FORMAT", possible_values = &["JSON", "YAML", "TOML"], case_insensitive = true)]
    pub to: Option<Format>,

    /// List supported input formats.
    #[structopt(long)]
    pub list_input_formats: bool,

    /// List supported output formats.
    #[structopt(long)]
    pub list_output_formats: bool,

    /// Output to <FILE> instead of stdout.
    #[structopt(short, long, value_name = "FILE", parse(from_os_str))]
    pub output: Option<PathBuf>,

    /// Output as a pretty-printed string.
    #[structopt(long, value_name = "BOOLEAN")]
    pub pretty: Option<Option<bool>>,

    /// Input from <FILE>.
    #[structopt(value_name = "FILE", parse(from_os_str))]
    pub input: Option<PathBuf>,
}

impl Opt {
    /// Guess the input format from the extension of a input file.
    fn guess_input_format(mut self) -> Self {
        if let Some(ref f) = self.input {
            self.from = f.extension().and_then(OsStr::to_str).and_then(guess_format);
        }

        self
    }

    /// Guess the output format from the extension of a output file.
    fn guess_output_format(mut self) -> Self {
        if let Some(ref f) = self.output {
            self.to = f.extension().and_then(OsStr::to_str).and_then(guess_format);
        }

        self
    }

    /// Do processing related to options.
    pub fn process(mut self) -> Self {
        self = self.guess_input_format().guess_output_format();

        self
    }
}

/// Guess the format from the extension.
fn guess_format(ext: &str) -> Option<Format> {
    match ext {
        "json" => Some(Format::Json),
        "yaml" | "yml" => Some(Format::Yaml),
        "toml" => Some(Format::Toml),
        _ => None,
    }
}
