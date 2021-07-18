//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use std::fmt;
use std::str::FromStr;

use anyhow::{Error, Result};
use indexmap::IndexMap;

#[derive(Clone, Copy, Debug)]
pub enum Format {
    Cbor,
    Hjson,
    Json,
    Json5,
    MessagePack,
    Toml,
    Yaml,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Cbor => write!(f, "CBOR"),
            Format::Hjson => write!(f, "Hjson"),
            Format::Json => write!(f, "JSON"),
            Format::Json5 => write!(f, "JSON5"),
            Format::MessagePack => write!(f, "MessagePack"),
            Format::Toml => write!(f, "TOML"),
            Format::Yaml => write!(f, "YAML"),
        }
    }
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(format: &str) -> Result<Self> {
        match format.to_ascii_lowercase().as_str() {
            "cbor" => Ok(Format::Cbor),
            "hjson" => Ok(Format::Hjson),
            "json" => Ok(Format::Json),
            "json5" => Ok(Format::Json5),
            "messagepack" => Ok(Format::MessagePack),
            "toml" => Ok(Format::Toml),
            "yaml" => Ok(Format::Yaml),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Map(IndexMap<String, Value>),
}
