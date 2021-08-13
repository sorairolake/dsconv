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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Cbor => write!(f, "CBOR"),
            Format::Hjson => write!(f, "Hjson"),
            Format::Json => write!(f, "JSON"),
            Format::Json5 => write!(f, "JSON5"),
            Format::MessagePack => write!(f, "MessagePack"),
            Format::Ron => write!(f, "RON"),
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
            "ron" => Ok(Format::Ron),
            "toml" => Ok(Format::Toml),
            "yaml" | "yml" => Ok(Format::Yaml),
            _ => Err(anyhow!("Unknown format: {}", format)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum I {
    Pos(u64),
    Neg(i64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Integer {
    i: I,
}

impl Integer {
    pub fn as_i64(&self) -> Option<i64> {
        match self.i {
            I::Pos(u) if u <= i64::MAX as u64 => Some(u as i64),
            I::Neg(i) => Some(i),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self.i {
            I::Pos(u) => Some(u),
            _ => None,
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.i {
            I::Pos(u) => Display::fmt(&u, f),
            I::Neg(i) => Display::fmt(&i, f),
        }
    }
}

impl From<i64> for Integer {
    fn from(integer: i64) -> Self {
        if integer < 0 {
            Integer { i: I::Neg(integer) }
        } else {
            Integer {
                i: I::Pos(integer as u64),
            }
        }
    }
}

impl From<u64> for Integer {
    fn from(integer: u64) -> Self {
        Integer { i: I::Pos(integer) }
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
