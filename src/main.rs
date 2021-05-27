//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

mod cli;
mod convert;
mod value;

use std::convert::TryInto;
use std::fs;
use std::io::{self, Read};

use anyhow::{bail, Context, Result};
use serde_json::Value as Json;
use serde_yaml::Value as Yaml;
use structopt::StructOpt;
use toml::Value as Toml;

use crate::cli::Opt;
use crate::value::{Format, Value};

fn main() -> Result<()> {
    let opt = Opt::from_args();

    if let Some(s) = opt.generate_completion {
        Opt::generate_completion(s);

        return Ok(());
    }

    if opt.list_input_formats {
        println!("{}", Format::Hjson);
        println!("{}", Format::Json);
        println!("{}", Format::Json5);
        println!("{}", Format::Toml);
        println!("{}", Format::Yaml);

        return Ok(());
    }
    if opt.list_output_formats {
        println!("{}", Format::Json);
        println!("{}", Format::Toml);
        println!("{}", Format::Yaml);

        return Ok(());
    }

    let input = match opt.input {
        Some(ref f) => fs::read_to_string(f)
            .with_context(|| format!("Failed to read a string from {}", f.display()))?,
        None if atty::isnt(atty::Stream::Stdin) => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .context("Failed to read a string from stdin")?;
            buf
        }
        _ => bail!("Input from tty is invalid"),
    };

    let opt = opt.process();

    if opt.from.is_none() || opt.to.is_none() {
        bail!("Unable to determine input and/or output format");
    }

    let ir_value: Value = match opt.from {
        Some(Format::Hjson) => {
            let hjson: Json = deser_hjson::from_str(&input)
                .context("Failed to deserialize from a string of Hjson")?;

            hjson.into()
        }
        Some(Format::Json) => {
            let json: Json = serde_json::from_str(&input)
                .context("Failed to deserialize from a string of JSON")?;

            json.into()
        }
        Some(Format::Json5) => {
            let json5: Json =
                json5::from_str(&input).context("Failed to deserialize from a string of JSON5")?;

            json5.into()
        }
        Some(Format::Toml) => {
            let toml: Toml =
                toml::from_str(&input).context("Failed to deserialize from a string of TOML")?;

            toml.into()
        }
        Some(Format::Yaml) => {
            let yaml: Yaml = serde_yaml::from_str(&input)
                .context("Failed to deserialize from a string of YAML")?;

            yaml.try_into()
                .context("Failed to convert from a YAML value")?
        }
        None => unreachable!(),
    };

    let output = match opt.to {
        Some(Format::Json) => {
            let json: Json = ir_value
                .try_into()
                .context("Failed to convert to a JSON value")?;

            if opt.is_pretty_print() {
                serde_json::to_string_pretty(&json)
                    .context("Failed to serialize to a pretty-printed string of JSON")?
                    + "\n"
            } else {
                serde_json::to_string(&json).context("Failed to serialize to a string of JSON")?
                    + "\n"
            }
        }
        Some(Format::Toml) => {
            let toml: Toml = ir_value
                .try_into()
                .context("Failed to convert to a TOML value")?;

            if opt.is_pretty_print() {
                toml::to_string_pretty(&toml)
                    .context("Failed to serialize to a pretty-printed string of TOML")?
            } else {
                toml::to_string(&toml).context("Failed to serialize to a string of TOML")?
            }
        }
        Some(Format::Yaml) => {
            let yaml: Yaml = ir_value.into();

            serde_yaml::to_string(&yaml).context("Failed to serialize to a string of YAML")?
        }
        _ => unreachable!(),
    };

    match opt.output {
        Some(ref f) => {
            fs::write(f, output).with_context(|| format!("Failed to write to {}", f.display()))?
        }
        None => print!("{}", output),
    }

    Ok(())
}
