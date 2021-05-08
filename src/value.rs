//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use std::fmt;
use std::str::FromStr;

use anyhow::{Error, Result};
use indexmap::IndexMap;

#[derive(Debug)]
pub enum Format {
    Json,
    Yaml,
    Toml,
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(format: &str) -> Result<Self> {
        match format.to_ascii_lowercase().as_str() {
            "json" => Ok(Format::Json),
            "yaml" => Ok(Format::Yaml),
            "toml" => Ok(Format::Toml),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Json => write!(f, "JSON"),
            Format::Yaml => write!(f, "YAML"),
            Format::Toml => write!(f, "TOML"),
        }
    }
}

#[derive(Clone, Debug)]
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
