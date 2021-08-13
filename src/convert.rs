//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use std::convert::{From, TryFrom, TryInto};

use anyhow::{anyhow, Context, Result};
use rmpv::Value as MessagePack;
use ron::Value as Ron;
use serde_cbor::Value as Cbor;
use serde_json::Value as Json;
use serde_yaml::Value as Yaml;
use toml::Value as Toml;

use crate::value::Value;

impl TryFrom<Cbor> for Value {
    type Error = anyhow::Error;

    fn try_from(value: Cbor) -> Result<Self> {
        match value {
            Cbor::Null => Ok(Value::Null),
            Cbor::Bool(bool) => Ok(Value::Bool(bool)),
            Cbor::Integer(int) => Ok(Value::Integer(i64::try_from(int).map_or_else(
                |_| u64::try_from(int).expect("Invalid integer as IR").into(),
                |s| s.into(),
            ))),
            Cbor::Float(float) => Ok(Value::Float(float)),
            Cbor::Bytes(_) => Err(anyhow!("A byte string cannot be converted")),
            Cbor::Text(str) => Ok(Value::String(str)),
            Cbor::Array(arr) => Ok(Value::Array(
                arr.into_iter()
                    .map(|v| v.try_into())
                    .collect::<Result<Vec<_>>>()?,
            )),
            Cbor::Map(map) => Ok(Value::Map(
                map.keys()
                    .cloned()
                    .map(|k| serde_cbor::value::from_value(k).context("The key is not a string"))
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .zip(
                        map.values()
                            .cloned()
                            .map(|v| v.try_into())
                            .collect::<Result<Vec<_>>>()?
                            .into_iter(),
                    )
                    .collect(),
            )),
            Cbor::Tag(..) => Err(anyhow!("A semantic tag cannot be converted")),
            _ => unreachable!(),
        }
    }
}

impl From<Json> for Value {
    fn from(value: Json) -> Self {
        match value {
            Json::Null => Value::Null,
            Json::Bool(bool) => Value::Bool(bool),
            Json::Number(num) => {
                if let Some(sint) = num.as_i64() {
                    return Value::Integer(sint.into());
                }
                if let Some(uint) = num.as_u64() {
                    return Value::Integer(uint.into());
                }

                // Return value is definitely Some(T).
                Value::Float(num.as_f64().expect("Invalid number as IR"))
            }
            Json::String(str) => Value::String(str),
            Json::Array(arr) => Value::Array(arr.into_iter().map(|v| v.into()).collect()),
            Json::Object(obj) => Value::Map(obj.into_iter().map(|(k, v)| (k, v.into())).collect()),
        }
    }
}

impl TryFrom<MessagePack> for Value {
    type Error = anyhow::Error;

    fn try_from(value: MessagePack) -> Result<Self> {
        match value {
            MessagePack::Nil => Ok(Value::Null),
            MessagePack::Boolean(bool) => Ok(Value::Bool(bool)),
            MessagePack::Integer(int) => Ok(Value::Integer(int.as_i64().map_or_else(
                || int.as_u64().expect("Invalid integer as IR").into(),
                |s| s.into(),
            ))),
            MessagePack::F32(single) => Ok(Value::Float(single.into())),
            MessagePack::F64(double) => Ok(Value::Float(double)),
            MessagePack::String(msgpack_str) => Ok(Value::String(
                msgpack_str
                    .as_str()
                    .with_context(|| {
                        format!(
                            "The string contains invalid UTF-8 sequence: {}",
                            msgpack_str
                        )
                    })?
                    .to_string(),
            )),
            MessagePack::Binary(_) => Err(anyhow!("A byte array cannot be converted")),
            MessagePack::Array(arr) => Ok(Value::Array(
                arr.into_iter()
                    .map(|v| v.try_into())
                    .collect::<Result<Vec<_>>>()?,
            )),
            MessagePack::Map(map) => Ok(Value::Map(
                map.iter()
                    .map(|(k, _)| k)
                    .map(|k| k.as_str().context("The key is not a string"))
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .map(|k| k.to_string())
                    .zip(
                        map.iter()
                            .map(|(_, v)| v)
                            .cloned()
                            .map(|v| v.try_into())
                            .collect::<Result<Vec<_>>>()?
                            .into_iter(),
                    )
                    .collect(),
            )),
            MessagePack::Ext(..) => Err(anyhow!("An extension cannot be converted")),
        }
    }
}

impl TryFrom<Ron> for Value {
    type Error = anyhow::Error;

    fn try_from(value: Ron) -> Result<Self> {
        match value {
            Ron::Bool(bool) => Ok(Value::Bool(bool)),
            Ron::Char(char) => Ok(Value::String(char.into())),
            Ron::Map(map) => Ok(Value::Map(
                map.keys()
                    .cloned()
                    .map(|k| k.into_rust().context("The key is not a string"))
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .zip(
                        map.values()
                            .cloned()
                            .map(|v| v.try_into())
                            .collect::<Result<Vec<_>>>()?
                            .into_iter(),
                    )
                    .collect(),
            )),
            Ron::Number(num) => Ok(num.as_i64().map_or_else(
                || Value::Float(num.as_f64().expect("Invalid number as IR")),
                |i| Value::Integer(i.into()),
            )),
            Ron::Option(_) => Err(anyhow!("The Option type cannot be converted")),
            Ron::String(str) => Ok(Value::String(str)),
            Ron::Seq(seq) => Ok(Value::Array(
                seq.into_iter()
                    .map(|v| v.try_into())
                    .collect::<Result<Vec<_>>>()?,
            )),
            Ron::Unit => Err(anyhow!("The unit type cannot be converted")),
        }
    }
}

impl From<Toml> for Value {
    fn from(value: Toml) -> Self {
        match value {
            Toml::String(str) => Value::String(str),
            Toml::Integer(int) => Value::Integer(int.into()),
            Toml::Float(float) => Value::Float(float),
            Toml::Boolean(bool) => Value::Bool(bool),
            Toml::Datetime(dt) => Value::String(dt.to_string()),
            Toml::Array(arr) => Value::Array(arr.into_iter().map(|v| v.into()).collect()),
            Toml::Table(table) => {
                Value::Map(table.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
        }
    }
}

impl TryFrom<Yaml> for Value {
    type Error = anyhow::Error;

    fn try_from(value: Yaml) -> Result<Self> {
        match value {
            Yaml::Null => Ok(Value::Null),
            Yaml::Bool(bool) => Ok(Value::Bool(bool)),
            Yaml::Number(num) => {
                if let Some(sint) = num.as_i64() {
                    return Ok(Value::Integer(sint.into()));
                }
                if let Some(uint) = num.as_u64() {
                    return Ok(Value::Integer(uint.into()));
                }

                // Return value is definitely Some(T).
                Ok(Value::Float(num.as_f64().expect("Invalid number as IR")))
            }
            Yaml::String(str) => Ok(Value::String(str)),
            Yaml::Sequence(seq) => Ok(Value::Array(
                seq.into_iter()
                    .map(|v| v.try_into())
                    .collect::<Result<Vec<_>>>()?,
            )),
            Yaml::Mapping(map) => Ok(Value::Map(
                map.iter()
                    .map(|(k, _)| k)
                    .map(|k| k.as_str().context("The key is not a string"))
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .map(|k| k.to_string())
                    .zip(
                        map.iter()
                            .map(|(_, v)| v)
                            .cloned()
                            .map(|v| v.try_into())
                            .collect::<Result<Vec<_>>>()?
                            .into_iter(),
                    )
                    .collect(),
            )),
        }
    }
}

impl From<Value> for Cbor {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Cbor::Null,
            Value::Bool(bool) => Cbor::Bool(bool),
            Value::Integer(int) => Cbor::Integer(int.as_i64().map_or_else(
                || int.as_u64().expect("Invalid integer as CBOR").into(),
                |s| s.into(),
            )),
            Value::Float(float) => Cbor::Float(float),
            Value::String(str) => Cbor::Text(str),
            Value::Array(arr) => Cbor::Array(arr.into_iter().map(|v| v.into()).collect()),
            Value::Map(map) => {
                Cbor::Map(map.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
            }
        }
    }
}

impl TryFrom<Value> for Json {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Null => Ok(Json::Null),
            Value::Bool(bool) => Ok(Json::Bool(bool)),
            Value::Integer(int) => Ok(Json::Number(int.as_i64().map_or_else(
                || int.as_u64().expect("Invalid integer as JSON").into(),
                |s| s.into(),
            ))),
            Value::Float(float) => Ok(Json::Number(
                serde_json::Number::from_f64(float).with_context(|| {
                    format!("Infinite or NaN values are not allowed: {}", float)
                })?,
            )),
            Value::String(str) => Ok(Json::String(str)),
            Value::Array(arr) => Ok(Json::Array(
                arr.into_iter()
                    .map(|v| v.try_into())
                    .collect::<Result<Vec<_>>>()?,
            )),
            Value::Map(map) => Ok(Json::Object(
                map.keys()
                    .cloned()
                    .zip(
                        map.values()
                            .cloned()
                            .map(|v| v.try_into())
                            .collect::<Result<Vec<_>>>()?
                            .into_iter(),
                    )
                    .collect(),
            )),
        }
    }
}

impl From<Value> for MessagePack {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => MessagePack::Nil,
            Value::Bool(bool) => MessagePack::Boolean(bool),
            Value::Integer(int) => MessagePack::Integer(int.as_i64().map_or_else(
                || int.as_u64().expect("Invalid integer as MessagePack").into(),
                |s| s.into(),
            )),
            Value::Float(float) => MessagePack::F64(float),
            Value::String(str) => MessagePack::String(str.into()),
            Value::Array(arr) => MessagePack::Array(arr.into_iter().map(|v| v.into()).collect()),
            Value::Map(map) => {
                MessagePack::Map(map.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
            }
        }
    }
}

impl TryFrom<Value> for Toml {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Null => Err(anyhow!("Null does not exist")),
            Value::Bool(bool) => Ok(Toml::Boolean(bool)),
            Value::Integer(int) => {
                Ok(Toml::Integer(int.as_i64().with_context(|| {
                    format!("Out of range of integer: {}", int)
                })?))
            }
            Value::Float(float) => Ok(Toml::Float(float)),
            Value::String(str) => Ok(str.parse().map_or(Toml::String(str), Toml::Datetime)),
            Value::Array(arr) => Ok(Toml::Array(
                arr.into_iter()
                    .map(|v| v.try_into())
                    .collect::<Result<Vec<_>>>()?,
            )),
            Value::Map(map) => Ok(Toml::Table(
                map.keys()
                    .cloned()
                    .zip(
                        map.values()
                            .cloned()
                            .map(|v| v.try_into())
                            .collect::<Result<Vec<_>>>()?
                            .into_iter(),
                    )
                    .collect(),
            )),
        }
    }
}

impl From<Value> for Yaml {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Yaml::Null,
            Value::Bool(bool) => Yaml::Bool(bool),
            Value::Integer(int) => Yaml::Number(int.as_i64().map_or_else(
                || int.as_u64().expect("Invalid integer as YAML").into(),
                |s| s.into(),
            )),
            Value::Float(float) => Yaml::Number(float.into()),
            Value::String(str) => Yaml::String(str),
            Value::Array(arr) => Yaml::Sequence(arr.into_iter().map(|v| v.into()).collect()),
            Value::Map(map) => {
                Yaml::Mapping(map.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cbor2ir() {
        assert_eq!(TryInto::<Value>::try_into(Cbor::Null).unwrap(), Value::Null);
        assert_eq!(
            TryInto::<Value>::try_into(Cbor::Bool(bool::default())).unwrap(),
            Value::Bool(bool::default())
        );
        assert_eq!(
            TryInto::<Value>::try_into(Cbor::Integer(i64::MIN.into())).unwrap(),
            Value::Integer(i64::MIN.into())
        );
        assert_eq!(
            TryInto::<Value>::try_into(Cbor::Integer(u64::MAX.into())).unwrap(),
            Value::Integer(u64::MAX.into())
        );
        assert_eq!(
            TryInto::<Value>::try_into(Cbor::Float(f64::default())).unwrap(),
            Value::Float(f64::default())
        );
        assert!(TryInto::<Value>::try_into(Cbor::Bytes(vec![u8::MIN])).is_err());
        assert_eq!(
            TryInto::<Value>::try_into(Cbor::Text(String::default())).unwrap(),
            Value::String(String::default())
        );
        assert_eq!(
            TryInto::<Value>::try_into(Cbor::Array(vec![Cbor::Null])).unwrap(),
            Value::Array(vec![Value::Null])
        );
        assert_eq!(
            TryInto::<Value>::try_into(Cbor::Map(
                vec![(Cbor::Text(String::default()), Cbor::Null)]
                    .into_iter()
                    .collect()
            ))
            .unwrap(),
            Value::Map(vec![(String::default(), Value::Null)].into_iter().collect())
        );
        assert!(TryInto::<Value>::try_into(Cbor::Tag(u64::MIN, Box::new(Cbor::Null))).is_err());

        assert!(TryInto::<Value>::try_into(Cbor::Map(
            vec![(Cbor::Null, Cbor::Null)].into_iter().collect()
        ))
        .is_err());
    }

    #[test]
    fn json2ir() {
        assert_eq!(Into::<Value>::into(Json::Null), Value::Null);
        assert_eq!(
            Into::<Value>::into(Json::Bool(bool::default())),
            Value::Bool(bool::default())
        );
        assert_eq!(
            Into::<Value>::into(Json::Number(i64::MIN.into())),
            Value::Integer(i64::MIN.into())
        );
        assert_eq!(
            Into::<Value>::into(Json::Number(u64::MAX.into())),
            Value::Integer(u64::MAX.into())
        );
        assert_eq!(
            Into::<Value>::into(Json::Number(
                serde_json::Number::from_f64(f64::default()).unwrap()
            )),
            Value::Float(f64::default())
        );
        assert_eq!(
            Into::<Value>::into(Json::String(String::default())),
            Value::String(String::default())
        );
        assert_eq!(
            Into::<Value>::into(Json::Array(vec![Json::Null])),
            Value::Array(vec![Value::Null])
        );
        assert_eq!(
            Into::<Value>::into(Json::Object(
                vec![(String::default(), Json::Null)].into_iter().collect()
            )),
            Value::Map(vec![(String::default(), Value::Null)].into_iter().collect())
        );
    }

    #[test]
    fn messagepack2ir() {
        assert_eq!(
            TryInto::<Value>::try_into(MessagePack::Nil).unwrap(),
            Value::Null
        );
        assert_eq!(
            TryInto::<Value>::try_into(MessagePack::Boolean(bool::default())).unwrap(),
            Value::Bool(bool::default())
        );
        assert_eq!(
            TryInto::<Value>::try_into(MessagePack::Integer(i64::MIN.into())).unwrap(),
            Value::Integer(i64::MIN.into())
        );
        assert_eq!(
            TryInto::<Value>::try_into(MessagePack::Integer(u64::MAX.into())).unwrap(),
            Value::Integer(u64::MAX.into())
        );
        assert_eq!(
            TryInto::<Value>::try_into(MessagePack::F32(f32::default())).unwrap(),
            Value::Float(f64::default())
        );
        assert_eq!(
            TryInto::<Value>::try_into(MessagePack::F64(f64::default())).unwrap(),
            Value::Float(f64::default())
        );
        assert_eq!(
            TryInto::<Value>::try_into(MessagePack::String(String::default().into())).unwrap(),
            Value::String(String::default())
        );
        assert!(TryInto::<Value>::try_into(MessagePack::Binary(vec![u8::MIN])).is_err());
        assert_eq!(
            TryInto::<Value>::try_into(MessagePack::Array(vec![MessagePack::Nil])).unwrap(),
            Value::Array(vec![Value::Null])
        );
        assert_eq!(
            TryInto::<Value>::try_into(MessagePack::Map(
                vec![(
                    MessagePack::String(String::default().into()),
                    MessagePack::Nil
                )]
                .into_iter()
                .collect()
            ))
            .unwrap(),
            Value::Map(vec![(String::default(), Value::Null)].into_iter().collect())
        );
        assert!(
            TryInto::<Value>::try_into(MessagePack::Ext(i8::default(), vec![u8::MIN])).is_err()
        );

        assert!(TryInto::<Value>::try_into(MessagePack::Map(
            vec![(MessagePack::Nil, MessagePack::Nil)]
                .into_iter()
                .collect()
        ))
        .is_err());
    }

    #[test]
    fn ron2ir() {
        assert_eq!(
            TryInto::<Value>::try_into(Ron::Bool(bool::default())).unwrap(),
            Value::Bool(bool::default())
        );
        assert_eq!(
            TryInto::<Value>::try_into(Ron::Char(char::default())).unwrap(),
            Value::String(char::default().into())
        );
        assert_eq!(
            TryInto::<Value>::try_into(Ron::Map(
                vec![(Ron::String(String::default()), Ron::Bool(bool::default()))]
                    .into_iter()
                    .collect()
            ))
            .unwrap(),
            Value::Map(
                vec![(String::default(), Value::Bool(bool::default()))]
                    .into_iter()
                    .collect()
            )
        );
        assert_eq!(
            TryInto::<Value>::try_into(Ron::Number(i64::default().into())).unwrap(),
            Value::Integer(i64::default().into())
        );
        assert_eq!(
            TryInto::<Value>::try_into(Ron::Number(f64::default().into())).unwrap(),
            Value::Float(f64::default())
        );
        assert!(TryInto::<Value>::try_into(Ron::Option(Option::default())).is_err());
        assert_eq!(
            TryInto::<Value>::try_into(Ron::String(String::default())).unwrap(),
            Value::String(String::default())
        );
        assert_eq!(
            TryInto::<Value>::try_into(Ron::Seq(vec![Ron::Bool(bool::default())])).unwrap(),
            Value::Array(vec![Value::Bool(bool::default())])
        );
        assert!(TryInto::<Value>::try_into(Ron::Unit).is_err());

        assert!(TryInto::<Value>::try_into(Ron::Map(
            vec![(Ron::Bool(bool::default()), Ron::Bool(bool::default()))]
                .into_iter()
                .collect()
        ))
        .is_err());
    }

    #[test]
    fn toml2ir() {
        assert_eq!(
            Into::<Value>::into(Toml::String(String::default())),
            Value::String(String::default())
        );
        assert_eq!(
            Into::<Value>::into(Toml::Integer(i64::default())),
            Value::Integer(i64::default().into())
        );
        assert_eq!(
            Into::<Value>::into(Toml::Float(f64::default())),
            Value::Float(f64::default())
        );
        assert_eq!(
            Into::<Value>::into(Toml::Boolean(bool::default())),
            Value::Bool(bool::default())
        );
        assert_eq!(
            Into::<Value>::into(Toml::Datetime("1970-01-01T00:00:00Z".parse().unwrap())),
            Value::String("1970-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            Into::<Value>::into(Toml::Array(vec![Toml::Boolean(bool::default())])),
            Value::Array(vec![Value::Bool(bool::default())])
        );
        assert_eq!(
            Into::<Value>::into(Toml::Table(
                vec![(String::default(), Toml::Boolean(bool::default()))]
                    .into_iter()
                    .collect()
            )),
            Value::Map(
                vec![(String::default(), Value::Bool(bool::default()))]
                    .into_iter()
                    .collect()
            )
        );
    }

    #[test]
    fn yaml2ir() {
        assert_eq!(TryInto::<Value>::try_into(Yaml::Null).unwrap(), Value::Null);
        assert_eq!(
            TryInto::<Value>::try_into(Yaml::Bool(bool::default())).unwrap(),
            Value::Bool(bool::default())
        );
        assert_eq!(
            TryInto::<Value>::try_into(Yaml::Number(i64::MIN.into())).unwrap(),
            Value::Integer(i64::MIN.into())
        );
        assert_eq!(
            TryInto::<Value>::try_into(Yaml::Number(u64::MAX.into())).unwrap(),
            Value::Integer(u64::MAX.into())
        );
        assert_eq!(
            TryInto::<Value>::try_into(Yaml::Number(f64::default().into())).unwrap(),
            Value::Float(f64::default())
        );
        assert_eq!(
            TryInto::<Value>::try_into(Yaml::String(String::default())).unwrap(),
            Value::String(String::default())
        );
        assert_eq!(
            TryInto::<Value>::try_into(Yaml::Sequence(vec![Yaml::Null])).unwrap(),
            Value::Array(vec![Value::Null])
        );
        assert_eq!(
            TryInto::<Value>::try_into(Yaml::Mapping(
                vec![(Yaml::String(String::default()), Yaml::Null)]
                    .into_iter()
                    .collect()
            ))
            .unwrap(),
            Value::Map(vec![(String::default(), Value::Null)].into_iter().collect())
        );

        assert!(TryInto::<Value>::try_into(Yaml::Mapping(
            vec![(Yaml::Null, Yaml::Null)].into_iter().collect()
        ))
        .is_err());
    }

    #[test]
    fn ir2cbor() {
        assert_eq!(Into::<Cbor>::into(Value::Null), Cbor::Null);
        assert_eq!(
            Into::<Cbor>::into(Value::Bool(bool::default())),
            Cbor::Bool(bool::default())
        );
        assert_eq!(
            Into::<Cbor>::into(Value::Integer(i64::MIN.into())),
            Cbor::Integer(i64::MIN.into())
        );
        assert_eq!(
            Into::<Cbor>::into(Value::Integer(u64::MAX.into())),
            Cbor::Integer(u64::MAX.into())
        );
        assert_eq!(
            Into::<Cbor>::into(Value::Float(f64::default())),
            Cbor::Float(f64::default())
        );
        assert_eq!(
            Into::<Cbor>::into(Value::String(String::default())),
            Cbor::Text(String::default())
        );
        assert_eq!(
            Into::<Cbor>::into(Value::Array(vec![Value::Null])),
            Cbor::Array(vec![Cbor::Null])
        );
        assert_eq!(
            Into::<Cbor>::into(Value::Map(
                vec![(String::default(), Value::Null)].into_iter().collect()
            )),
            Cbor::Map(
                vec![(Cbor::Text(String::default()), Cbor::Null)]
                    .into_iter()
                    .collect()
            )
        );
    }

    #[test]
    fn ir2json() {
        assert_eq!(TryInto::<Json>::try_into(Value::Null).unwrap(), Json::Null);
        assert_eq!(
            TryInto::<Json>::try_into(Value::Bool(bool::default())).unwrap(),
            Json::Bool(bool::default())
        );
        assert_eq!(
            TryInto::<Json>::try_into(Value::Integer(i64::MIN.into())).unwrap(),
            Json::Number(i64::MIN.into())
        );
        assert_eq!(
            TryInto::<Json>::try_into(Value::Integer(u64::MAX.into())).unwrap(),
            Json::Number(u64::MAX.into())
        );
        assert_eq!(
            TryInto::<Json>::try_into(Value::Float(f64::default())).unwrap(),
            Json::Number(serde_json::Number::from_f64(f64::default()).unwrap())
        );
        assert_eq!(
            TryInto::<Json>::try_into(Value::String(String::default())).unwrap(),
            Json::String(String::default())
        );
        assert_eq!(
            TryInto::<Json>::try_into(Value::Array(vec![Value::Null])).unwrap(),
            Json::Array(vec![Json::Null])
        );
        assert_eq!(
            TryInto::<Json>::try_into(Value::Map(
                vec![(String::default(), Value::Null)].into_iter().collect()
            ))
            .unwrap(),
            Json::Object(vec![(String::default(), Json::Null)].into_iter().collect())
        );

        assert!(TryInto::<Json>::try_into(Value::Float(f64::NAN)).is_err());
        assert!(TryInto::<Json>::try_into(Value::Float(f64::INFINITY)).is_err());
        assert!(TryInto::<Json>::try_into(Value::Float(f64::NEG_INFINITY)).is_err());
    }

    #[test]
    fn ir2messagepack() {
        assert_eq!(Into::<MessagePack>::into(Value::Null), MessagePack::Nil);
        assert_eq!(
            Into::<MessagePack>::into(Value::Bool(bool::default())),
            MessagePack::Boolean(bool::default())
        );
        assert_eq!(
            Into::<MessagePack>::into(Value::Integer(i64::MIN.into())),
            MessagePack::Integer(i64::MIN.into())
        );
        assert_eq!(
            Into::<MessagePack>::into(Value::Integer(u64::MAX.into())),
            MessagePack::Integer(u64::MAX.into())
        );
        assert_eq!(
            Into::<MessagePack>::into(Value::Float(f64::default())),
            MessagePack::F64(f64::default())
        );
        assert_eq!(
            Into::<MessagePack>::into(Value::String(String::default())),
            MessagePack::String(String::default().into())
        );
        assert_eq!(
            Into::<MessagePack>::into(Value::Array(vec![Value::Null])),
            MessagePack::Array(vec![MessagePack::Nil])
        );
        assert_eq!(
            Into::<MessagePack>::into(Value::Map(
                vec![(String::default(), Value::Null)].into_iter().collect()
            )),
            MessagePack::Map(
                vec![(
                    MessagePack::String(String::default().into()),
                    MessagePack::Nil
                )]
                .into_iter()
                .collect()
            )
        );
    }

    #[test]
    fn ir2toml() {
        assert!(TryInto::<Toml>::try_into(Value::Null).is_err());
        assert_eq!(
            TryInto::<Toml>::try_into(Value::Bool(bool::default())).unwrap(),
            Toml::Boolean(bool::default())
        );
        assert_eq!(
            TryInto::<Toml>::try_into(Value::Integer(i64::default().into())).unwrap(),
            Toml::Integer(i64::default())
        );
        assert_eq!(
            TryInto::<Toml>::try_into(Value::Float(f64::default())).unwrap(),
            Toml::Float(f64::default())
        );
        assert_eq!(
            TryInto::<Toml>::try_into(Value::String("1970-01-01T00:00:00Z".to_string())).unwrap(),
            Toml::Datetime("1970-01-01T00:00:00Z".parse().unwrap())
        );
        assert_eq!(
            TryInto::<Toml>::try_into(Value::String(String::default())).unwrap(),
            Toml::String(String::default())
        );
        assert_eq!(
            TryInto::<Toml>::try_into(Value::Array(vec![Value::Bool(bool::default())])).unwrap(),
            Toml::Array(vec![Toml::Boolean(bool::default())])
        );
        assert_eq!(
            TryInto::<Toml>::try_into(Value::Map(
                vec![(String::default(), Value::Bool(bool::default()))]
                    .into_iter()
                    .collect()
            ))
            .unwrap(),
            Toml::Table(
                vec![(String::default(), Toml::Boolean(bool::default()))]
                    .into_iter()
                    .collect()
            )
        );

        assert!(TryInto::<Toml>::try_into(Value::Integer(u64::MAX.into())).is_err());
    }

    #[test]
    fn ir2yaml() {
        assert_eq!(Into::<Yaml>::into(Value::Null), Yaml::Null);
        assert_eq!(
            Into::<Yaml>::into(Value::Bool(bool::default())),
            Yaml::Bool(bool::default())
        );
        assert_eq!(
            Into::<Yaml>::into(Value::Integer(i64::MIN.into())),
            Yaml::Number(i64::MIN.into())
        );
        assert_eq!(
            Into::<Yaml>::into(Value::Integer(u64::MAX.into())),
            Yaml::Number(u64::MAX.into())
        );
        assert_eq!(
            Into::<Yaml>::into(Value::Float(f64::default())),
            Yaml::Number(f64::default().into())
        );
        assert_eq!(
            Into::<Yaml>::into(Value::String(String::default())),
            Yaml::String(String::default())
        );
        assert_eq!(
            Into::<Yaml>::into(Value::Array(vec![Value::Null])),
            Yaml::Sequence(vec![Yaml::Null])
        );
        assert_eq!(
            Into::<Yaml>::into(Value::Map(
                vec![(String::default(), Value::Null)].into_iter().collect()
            )),
            Yaml::Mapping(
                vec![(Yaml::String(String::default()), Yaml::Null)]
                    .into_iter()
                    .collect()
            )
        );
    }
}
