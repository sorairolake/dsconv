//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use std::fmt::{self, Display};

use indexmap::IndexMap;
use strum::{Display, EnumCount, EnumString, EnumVariantNames};

#[derive(Clone, Copy, Debug, Display, EnumCount, EnumString, EnumVariantNames)]
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

impl Format {
    pub const INPUT_VALUES: [&'static str; Self::COUNT] = [
        "cbor",
        "hjson",
        "json",
        "json5",
        "messagepack",
        "ron",
        "toml",
        "yaml",
    ];
    pub const OUTPUT_VALUES: [&'static str; Self::COUNT - 3] =
        ["cbor", "json", "messagepack", "toml", "yaml"];
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
