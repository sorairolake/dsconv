//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use std::fmt::{self, Display};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use indexmap::IndexMap;

#[derive(Clone, Copy, Debug)]
pub enum Format {
    Cbor,
    Hjson,
    Json,
    Json5,
    MessagePack,
    Ron,
    Toml,
    Yaml,
}

impl fmt::Display for Format {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Cbor => write!(fmt, "CBOR"),
            Format::Hjson => write!(fmt, "Hjson"),
            Format::Json => write!(fmt, "JSON"),
            Format::Json5 => write!(fmt, "JSON5"),
            Format::MessagePack => write!(fmt, "MessagePack"),
            Format::Ron => write!(fmt, "RON"),
            Format::Toml => write!(fmt, "TOML"),
            Format::Yaml => write!(fmt, "YAML"),
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
            "ron" => Ok(Format::Ron),
            "toml" => Ok(Format::Toml),
            "yaml" | "yml" => Ok(Format::Yaml),
            _ => Err(anyhow!("Unknown format: {}", format)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Int {
    Pos(u64),
    Neg(i64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Integer {
    int: Int,
}

impl Integer {
    pub fn as_i64(&self) -> Option<i64> {
        match self.int {
            Int::Pos(uint) if uint <= i64::MAX as u64 => Some(uint as i64),
            Int::Neg(sint) => Some(sint),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self.int {
            Int::Pos(uint) => Some(uint),
            _ => None,
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.int {
            Int::Pos(uint) => Display::fmt(&uint, fmt),
            Int::Neg(sint) => Display::fmt(&sint, fmt),
        }
    }
}

impl From<i64> for Integer {
    fn from(integer: i64) -> Self {
        if integer < 0 {
            Integer {
                int: Int::Neg(integer),
            }
        } else {
            Integer {
                int: Int::Pos(integer as u64),
            }
        }
    }
}

impl From<u64> for Integer {
    fn from(integer: u64) -> Self {
        Integer {
            int: Int::Pos(integer),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(Integer),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Map(IndexMap<String, Value>),
}
