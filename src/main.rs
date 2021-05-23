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

use anyhow::{bail, Result};
use serde_hjson::Value as Hjson;
use serde_json::Value as Json;
use serde_yaml::Value as Yaml;
use structopt::StructOpt;
use toml::Value as Toml;

use crate::cli::Opt;
use crate::value::{Format, Value};

fn main() -> Result<()> {
    let opt = Opt::from_args();

    if opt.generate_completions {
        Opt::generate_completions()?;

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
        println!("{}", Format::Hjson);
        println!("{}", Format::Json);
        println!("{}", Format::Toml);
        println!("{}", Format::Yaml);

        return Ok(());
    }

    let input = match opt.input {
        Some(ref f) => fs::read_to_string(f)?,
        None if atty::isnt(atty::Stream::Stdin) => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            buf
        }
        _ => bail!("Input from tty is invalid."),
    };

    let opt = opt.process();

    if opt.from.is_none() || opt.to.is_none() {
        bail!("Unable to determine input and/or output format.")
    }

    let ir_value: Value = match opt.from {
        Some(Format::Hjson) => {
            let hjson: Hjson = serde_hjson::from_str(&input)?;

            hjson.into()
        }
        Some(Format::Json) => {
            let json: Json = serde_json::from_str(&input)?;

            json.into()
        }
        Some(Format::Json5) => {
            let json5: Json = json5::from_str(&input)?;

            json5.into()
        }
        Some(Format::Toml) => {
            let toml: Toml = toml::from_str(&input)?;

            toml.into()
        }
        Some(Format::Yaml) => {
            let yaml: Yaml = serde_yaml::from_str(&input)?;

            yaml.try_into()?
        }
        None => unreachable!(),
    };

    let output = match opt.to {
        Some(Format::Hjson) => {
            let hjson: Hjson = ir_value.into();

            serde_hjson::to_string(&hjson)? + "\n"
        }
        Some(Format::Json) => {
            let json: Json = ir_value.try_into()?;

            if opt.is_pretty_print() {
                serde_json::to_string_pretty(&json)? + "\n"
            } else {
                serde_json::to_string(&json)? + "\n"
            }
        }
        Some(Format::Toml) => {
            let toml: Toml = ir_value.try_into()?;

            if opt.is_pretty_print() {
                toml::to_string_pretty(&toml)?
            } else {
                toml::to_string(&toml)?
            }
        }
        Some(Format::Yaml) => {
            let yaml: Yaml = ir_value.into();

            serde_yaml::to_string(&yaml)?
        }
        _ => unreachable!(),
    };

    match opt.output {
        Some(f) => fs::write(f, output)?,
        None => print!("{}", output),
    }

    Ok(())
}
