//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

mod cli;
mod config;
mod convert;
mod value;

use std::convert::TryInto;
use std::fs;
use std::io::{self, Read, Write};
use std::str;

use anyhow::{bail, Context, Result};
use rmpv::Value as MessagePack;
use serde_cbor::Value as Cbor;
use serde_json::Value as Json;
use serde_yaml::Value as Yaml;
use structopt::StructOpt;
use toml::Value as Toml;

use crate::cli::Opt;
use crate::value::{Format, Value};

fn main() -> Result<()> {
    let opt = Opt::from_args().apply_config()?;

    if let Some(s) = opt.generate_completion {
        if let Some(o) = opt.output {
            Opt::generate_completion_to_file(s, o)?;
        } else {
            Opt::generate_completion_to_stdout(s);
        }

        return Ok(());
    }

    if opt.list_input_formats {
        println!("{}", Format::Cbor);
        println!("{}", Format::Hjson);
        println!("{}", Format::Json);
        println!("{}", Format::Json5);
        println!("{}", Format::MessagePack);
        println!("{}", Format::Toml);
        println!("{}", Format::Yaml);

        return Ok(());
    }
    if opt.list_output_formats {
        println!("{}", Format::Cbor);
        println!("{}", Format::Json);
        println!("{}", Format::MessagePack);
        println!("{}", Format::Toml);
        println!("{}", Format::Yaml);

        return Ok(());
    }

    let input = match opt.input {
        Some(ref f) => {
            fs::read(f).with_context(|| format!("Failed to read bytes from {}", f.display()))?
        }
        None if atty::isnt(atty::Stream::Stdin) => {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("Failed to read bytes from stdin")?;
            buf
        }
        _ => bail!("Input from tty is invalid"),
    };

    let opt = opt.guess_input_format().guess_output_format();

    if opt.from.is_none() || opt.to.is_none() {
        bail!("Unable to determine input and/or output format");
    }

    let ir: Value = match opt.from {
        Some(Format::Cbor) => {
            let obj: Cbor = serde_cbor::from_slice(&input)
                .context("Failed to deserialize from a CBOR bytes")?;

            obj.try_into()
                .context("Failed to convert from a CBOR value")?
        }
        Some(Format::Hjson) => {
            let input =
                str::from_utf8(&input).context("Failed to convert from bytes to a string")?;
            let obj: Json = deser_hjson::from_str(&input)
                .context("Failed to deserialize from a Hjson string")?;

            obj.into()
        }
        Some(Format::Json) => {
            let input =
                str::from_utf8(&input).context("Failed to convert from bytes to a string")?;
            let obj: Json =
                serde_json::from_str(&input).context("Failed to deserialize from a JSON string")?;

            obj.into()
        }
        Some(Format::Json5) => {
            let input =
                str::from_utf8(&input).context("Failed to convert from bytes to a string")?;
            let obj: Json =
                json5::from_str(&input).context("Failed to deserialize from a JSON5 string")?;

            obj.into()
        }
        Some(Format::MessagePack) => {
            let obj: Vec<u8> = rmp_serde::from_read_ref(&input)
                .context("Failed to deserialize from a MessagePack bytes")?;

            rmpv::decode::read_value(&mut obj.as_slice())?
                .try_into()
                .context("Failed to convert from a MessagePack value")?
        }
        Some(Format::Toml) => {
            let input =
                str::from_utf8(&input).context("Failed to convert from bytes to a string")?;
            let obj: Toml =
                toml::from_str(&input).context("Failed to deserialize from a TOML string")?;

            obj.into()
        }
        Some(Format::Yaml) => {
            let input =
                str::from_utf8(&input).context("Failed to convert from bytes to a string")?;
            let obj: Yaml =
                serde_yaml::from_str(&input).context("Failed to deserialize from a YAML string")?;

            obj.try_into()
                .context("Failed to convert from a YAML value")?
        }
        None => unreachable!(),
    };

    let output = match opt.to {
        Some(Format::Cbor) => {
            let obj: Cbor = ir.into();

            serde_cbor::to_vec(&obj).context("Failed to serialize to a CBOR bytes")?
        }
        Some(Format::Json) => {
            let obj: Json = ir.try_into().context("Failed to convert to a JSON value")?;

            if opt.pretty.map_or(false, |p| p.unwrap_or(true)) {
                format!(
                    "{}\n",
                    serde_json::to_string_pretty(&obj)
                        .context("Failed to serialize to a JSON string")?
                )
                .into_bytes()
            } else {
                format!(
                    "{}\n",
                    serde_json::to_string(&obj).context("Failed to serialize to a JSON string")?
                )
                .into_bytes()
            }
        }
        Some(Format::MessagePack) => {
            let mut buf = Vec::new();
            let obj: MessagePack = ir.into();
            rmpv::encode::write_value(&mut buf, &obj)
                .context("Failed to write a MessagePack value to buffer")?;

            rmp_serde::to_vec(&buf).context("Failed to serialize to a MessagePack bytes")?
        }
        Some(Format::Toml) => {
            let obj: Toml = ir.try_into().context("Failed to convert to a TOML value")?;

            if opt.pretty.map_or(false, |p| p.unwrap_or(true)) {
                toml::to_string_pretty(&obj)
                    .context("Failed to serialize to a TOML string")?
                    .into_bytes()
            } else {
                toml::to_string(&obj)
                    .context("Failed to serialize to a TOML string")?
                    .into_bytes()
            }
        }
        Some(Format::Yaml) => {
            let obj: Yaml = ir.into();

            serde_yaml::to_string(&obj)
                .context("Failed to serialize to a YAML string")?
                .into_bytes()
        }
        _ => unreachable!(),
    };

    match opt.output {
        Some(ref f) => {
            fs::write(f, output).with_context(|| format!("Failed to write to {}", f.display()))?
        }
        None => io::stdout()
            .write_all(&output)
            .context("Failed to write to stdout")?,
    }

    Ok(())
}
