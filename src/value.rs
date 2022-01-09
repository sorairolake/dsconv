//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use std::fmt::{self, Display};

use clap::ArgEnum;
use indexmap::IndexMap;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(Clone, Copy, Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "UPPERCASE", ascii_case_insensitive)]
pub enum Format {
    Cbor,
    #[strum(to_string = "Hjson")]
    Hjson,
    Json,
    Json5,
    #[strum(serialize = "msgpack", to_string = "MessagePack")]
    MessagePack,
    Ron,
    Toml,
    #[strum(serialize = "yml", to_string = "YAML")]
    Yaml,
}

#[derive(ArgEnum, Clone, Copy)]
#[clap(rename_all = "lower")]
pub enum InputFormat {
    Cbor,
    Hjson,
    Json,
    Json5,
    MessagePack,
    Ron,
    Toml,
    Yaml,
}

impl From<InputFormat> for Format {
    fn from(value: InputFormat) -> Self {
        match value {
            InputFormat::Cbor => Self::Cbor,
            InputFormat::Hjson => Self::Hjson,
            InputFormat::Json => Self::Json,
            InputFormat::Json5 => Self::Json5,
            InputFormat::MessagePack => Self::MessagePack,
            InputFormat::Ron => Self::Ron,
            InputFormat::Toml => Self::Toml,
            InputFormat::Yaml => Self::Yaml,
        }
    }
}

#[derive(ArgEnum, Clone, Copy)]
#[clap(rename_all = "lower")]
pub enum OutputFormat {
    Cbor,
    Json,
    MessagePack,
    Toml,
    Yaml,
}

impl From<OutputFormat> for Format {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Cbor => Self::Cbor,
            OutputFormat::Json => Self::Json,
            OutputFormat::MessagePack => Self::MessagePack,
            OutputFormat::Toml => Self::Toml,
            OutputFormat::Yaml => Self::Yaml,
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
            Int::Pos(uint) if i64::try_from(uint).is_ok() => Some(uint as i64),
            Int::Neg(sint) => Some(sint),
            Int::Pos(_) => None,
        }
    }

    pub const fn as_u64(&self) -> Option<u64> {
        match self.int {
            Int::Pos(uint) => Some(uint),
            Int::Neg(_) => None,
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
            Self {
                int: Int::Neg(integer),
            }
        } else {
            Self {
                int: Int::Pos(integer as u64),
            }
        }
    }
}

impl From<u64> for Integer {
    fn from(integer: u64) -> Self {
        Self {
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

#[derive(ArgEnum, Clone, Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
#[clap(rename_all = "lower")]
pub enum Color {
    Auto,
    Always,
    Never,
}

impl Default for Color {
    fn default() -> Self {
        Self::Auto
    }
}
