//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

mod cli;
mod config;
mod convert;
mod macros;
mod value;

use std::ffi::OsStr;
use std::fs;
use std::io::{self, Read, Write};
use std::str;

use anyhow::{bail, ensure, Context, Result};
use bat::PrettyPrinter;
use clap::{ArgEnum, Parser};
use dialoguer::theme::ColorfulTheme;
use rmpv::Value as MessagePack;
use ron::Value as Ron;
use serde_cbor::Value as Cbor;
use serde_json::Value as Json;
use serde_yaml::Value as Yaml;
use toml::Value as Toml;

use crate::cli::Opt;
use crate::value::{Color, Format, InputFormat, OutputFormat, Value};

fn main() -> Result<()> {
    let opt = Opt::parse().apply_config()?;

    if let Some(shell) = opt.generate_completion {
        if let Some(out_dir) = opt.output {
            Opt::generate_completion_to(shell, out_dir)?;
        } else {
            Opt::generate_completion(shell);
        }

        return Ok(());
    }

    if opt.list_input_formats {
        for input_format in InputFormat::value_variants() {
            println!("{}", Format::from(*input_format));
        }

        return Ok(());
    }
    if opt.list_output_formats {
        for output_format in OutputFormat::value_variants() {
            println!("{}", Format::from(*output_format));
        }

        return Ok(());
    }

    let input = match opt.input {
        Some(ref file) => fs::read(file)
            .with_context(|| format!("Failed to read bytes from {}", file.display()))?,
        _ if atty::is(atty::Stream::Stdin) => {
            dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Input")
                .interact()
                .context("Failed to read a string from stdin")?
                .into_bytes()
        }
        _ => {
            let mut buf = Vec::new();
            io::stdin()
                .read_to_end(&mut buf)
                .context("Failed to read bytes from stdin")?;
            buf
        }
    };

    let ir: Value = match opt.from.map(Format::from).or_else(|| {
        opt.input.clone().and_then(|i| {
            i.extension()
                .and_then(OsStr::to_str)
                .and_then(|e| e.parse().ok())
        })
    }) {
        Some(Format::Cbor) => serde_cbor::from_slice::<Cbor>(&input)
            .context("Failed to deserialize from a CBOR bytes")?
            .try_into()
            .context("Failed to convert from a CBOR value")?,
        Some(Format::Hjson) => deser_hjson::from_str::<Json>(
            str::from_utf8(&input).context("Failed to convert from bytes to a string")?,
        )
        .context("Failed to deserialize from a Hjson string")?
        .into(),
        Some(Format::Json) => serde_json::from_str::<Json>(
            str::from_utf8(&input).context("Failed to convert from bytes to a string")?,
        )
        .context("Failed to deserialize from a JSON string")?
        .into(),
        Some(Format::Json5) => json5::from_str::<Json>(
            str::from_utf8(&input).context("Failed to convert from bytes to a string")?,
        )
        .context("Failed to deserialize from a JSON5 string")?
        .into(),
        Some(Format::MessagePack) => rmpv::decode::read_value(
            &mut rmp_serde::from_read_ref::<_, Vec<u8>>(&input)
                .context("Failed to deserialize from a MessagePack bytes")?
                .as_slice(),
        )?
        .try_into()
        .context("Failed to convert from a MessagePack value")?,
        Some(Format::Ron) => ron::from_str::<Ron>(
            str::from_utf8(&input).context("Failed to convert from bytes to a string")?,
        )
        .context("Failed to deserialize from a RON string")?
        .try_into()
        .context("Failed to convert from a RON value")?,
        Some(Format::Toml) => toml::from_str::<Toml>(
            str::from_utf8(&input).context("Failed to convert from bytes to a string")?,
        )
        .context("Failed to deserialize from a TOML string")?
        .into(),
        Some(Format::Yaml) => serde_yaml::from_str::<Yaml>(
            str::from_utf8(&input).context("Failed to convert from bytes to a string")?,
        )
        .context("Failed to deserialize from a YAML string")?
        .try_into()
        .context("Failed to convert from a YAML value")?,
        None => bail!("Unable to determine input format"),
    };

    let output = match opt.to.map(Format::from).or_else(|| {
        opt.output.clone().and_then(|o| {
            o.extension()
                .and_then(OsStr::to_str)
                .and_then(|e| e.parse().ok())
        })
    }) {
        Some(Format::Cbor) => {
            serde_cbor::to_vec(&Cbor::from(ir)).context("Failed to serialize to a CBOR bytes")?
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
            rmpv::encode::write_value(&mut buf, &MessagePack::from(ir))
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
        Some(Format::Yaml) => serde_yaml::to_string(&Yaml::from(ir))
            .context("Failed to serialize to a YAML string")?
            .into_bytes(),
        _ => bail!("Unable to determine output format"),
    };

    if let Some(ref file) = opt.output {
        fs::write(file, output)
            .with_context(|| format!("Failed to write to {}", file.display()))?;
    } else {
        let is_colored_output = match opt.color {
            Color::Auto if atty::is(atty::Stream::Stdout) => true,
            Color::Always => true,
            _ => false,
        };
        if is_colored_output {
            let language = Format::from(opt.to.expect("Unable to determine output format"));
            ensure!(
                !matches!(language, Format::Cbor | Format::MessagePack),
                "{} cannot colored output",
                language
            );

            let language = language.to_string();
            PrettyPrinter::new()
                .input_from_bytes(&output)
                .language(&language)
                .print()
                .expect("Failed to colored output");
        } else {
            io::stdout()
                .write_all(&output)
                .context("Failed to write to stdout")?;
        }
    }

    Ok(())
}
