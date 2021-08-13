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
            Cbor::Bool(b) => Ok(Value::Bool(b)),
            Cbor::Integer(i) => {
                if let Ok(i) = i64::try_from(i) {
                    return Ok(Value::Integer(i.into()));
                }

                // Return value is definitely Ok(T).
                Ok(Value::Integer(
                    u64::try_from(i).expect("Invalid integer as IR").into(),
                ))
            }
            Cbor::Float(f) => Ok(Value::Float(f)),
            Cbor::Bytes(_) => Err(anyhow!("A byte string cannot be converted")),
            Cbor::Text(s) => Ok(Value::String(s)),
            Cbor::Array(arr) => {
                let arr: Result<Vec<_>> = arr.into_iter().map(|v| v.try_into()).collect();
                let arr = arr?;

                Ok(Value::Array(arr))
            }
            Cbor::Map(map) => {
                let keys: Result<Vec<_>> = map
                    .keys()
                    .cloned()
                    .map(|k| serde_cbor::value::from_value(k).context("The key is not a string"))
                    .collect();
                let keys = keys?;
                let values: Result<Vec<_>> = map.values().cloned().map(|v| v.try_into()).collect();
                let values = values?;

                Ok(Value::Map(
                    keys.into_iter().zip(values.into_iter()).collect(),
                ))
            }
            Cbor::Tag(..) => Err(anyhow!("A semantic tag cannot be converted")),
            _ => unreachable!(),
        }
    }
}

impl From<Json> for Value {
    fn from(value: Json) -> Self {
        match value {
            Json::Null => Value::Null,
            Json::Bool(b) => Value::Bool(b),
            Json::Number(n) => {
                if let Some(i) = n.as_i64() {
                    return Value::Integer(i.into());
                }
                if let Some(u) = n.as_u64() {
                    return Value::Integer(u.into());
                }

                // Return value is definitely Some(T).
                Value::Float(n.as_f64().expect("Invalid number as IR"))
            }
            Json::String(s) => Value::String(s),
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
            MessagePack::Boolean(b) => Ok(Value::Bool(b)),
            MessagePack::Integer(i) => {
                if let Some(i) = i.as_i64() {
                    return Ok(Value::Integer(i.into()));
                }

                // Return value is definitely Some(T).
                Ok(Value::Integer(
                    i.as_u64().expect("Invalid integer as IR").into(),
                ))
            }
            MessagePack::F32(f) => Ok(Value::Float(f.into())),
            MessagePack::F64(f) => Ok(Value::Float(f)),
            MessagePack::String(s) => {
                let s = s.as_str().with_context(|| {
                    format!("The string contains invalid UTF-8 sequence: {}", s)
                })?;

                Ok(Value::String(s.to_string()))
            }
            MessagePack::Binary(_) => Err(anyhow!("A byte array cannot be converted")),
            MessagePack::Array(arr) => {
                let arr: Result<Vec<_>> = arr.into_iter().map(|v| v.try_into()).collect();
                let arr = arr?;

                Ok(Value::Array(arr))
            }
            MessagePack::Map(map) => {
                let (keys, values): (Vec<_>, Vec<_>) = map.into_iter().unzip();
                let keys: Result<Vec<_>> = keys
                    .iter()
                    .map(|k| k.as_str().context("The key is not a string"))
                    .collect();
                let keys = keys?;
                let values: Result<Vec<_>> = values.into_iter().map(|v| v.try_into()).collect();
                let values = values?;

                Ok(Value::Map(
                    keys.into_iter()
                        .map(|k| k.to_string())
                        .zip(values.into_iter())
                        .collect(),
                ))
            }
            MessagePack::Ext(..) => Err(anyhow!("An extension cannot be converted")),
        }
    }
}

impl TryFrom<Ron> for Value {
    type Error = anyhow::Error;

    fn try_from(value: Ron) -> Result<Self> {
        match value {
            Ron::Bool(b) => Ok(Value::Bool(b)),
            Ron::Char(c) => Ok(Value::String(c.into())),
            Ron::Map(map) => {
                let keys: Result<Vec<_>> = map
                    .keys()
                    .cloned()
                    .map(|k| k.into_rust().context("The key is not a string"))
                    .collect();
                let keys = keys?;
                let values: Result<Vec<_>> = map.values().cloned().map(|v| v.try_into()).collect();
                let values = values?;

                Ok(Value::Map(
                    keys.into_iter().zip(values.into_iter()).collect(),
                ))
            }
            Ron::Number(n) => {
                if let Some(i) = n.as_i64() {
                    return Ok(Value::Integer(i.into()));
                }

                // Return value is definitely Some(T).
                Ok(Value::Float(n.as_f64().expect("Invalid number as IR")))
            }
            Ron::Option(_) => Err(anyhow!("The Option type cannot be converted")),
            Ron::String(s) => Ok(Value::String(s)),
            Ron::Seq(seq) => {
                let arr: Result<Vec<_>> = seq.into_iter().map(|v| v.try_into()).collect();
                let arr = arr?;

                Ok(Value::Array(arr))
            }
            Ron::Unit => Err(anyhow!("The unit type cannot be converted")),
        }
    }
}

impl From<Toml> for Value {
    fn from(value: Toml) -> Self {
        match value {
            Toml::String(s) => Value::String(s),
            Toml::Integer(i) => Value::Integer(i.into()),
            Toml::Float(f) => Value::Float(f),
            Toml::Boolean(b) => Value::Bool(b),
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
            Yaml::Bool(b) => Ok(Value::Bool(b)),
            Yaml::Number(n) => {
                if let Some(i) = n.as_i64() {
                    return Ok(Value::Integer(i.into()));
                }
                if let Some(u) = n.as_u64() {
                    return Ok(Value::Integer(u.into()));
                }

                // Return value is definitely Some(T).
                Ok(Value::Float(n.as_f64().expect("Invalid number as IR")))
            }
            Yaml::String(s) => Ok(Value::String(s)),
            Yaml::Sequence(seq) => {
                let arr: Result<Vec<_>> = seq.into_iter().map(|v| v.try_into()).collect();
                let arr = arr?;

                Ok(Value::Array(arr))
            }
            Yaml::Mapping(map) => {
                let (keys, values): (Vec<_>, Vec<_>) = map.into_iter().unzip();
                let keys: Result<Vec<_>> = keys
                    .iter()
                    .map(|k| k.as_str().context("The key is not a string"))
                    .collect();
                let keys = keys?;
                let values: Result<Vec<_>> = values.into_iter().map(|v| v.try_into()).collect();
                let values = values?;

                Ok(Value::Map(
                    keys.into_iter()
                        .map(|k| k.to_string())
                        .zip(values.into_iter())
                        .collect(),
                ))
            }
        }
    }
}

impl From<Value> for Cbor {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Cbor::Null,
            Value::Bool(b) => Cbor::Bool(b),
            Value::Integer(i) => {
                if let Some(i) = i.as_i64() {
                    return Cbor::Integer(i.into());
                }

                // Return value is definitely Some(T).
                Cbor::Integer(i.as_u64().expect("Invalid integer as CBOR").into())
            }
            Value::Float(f) => Cbor::Float(f),
            Value::String(s) => Cbor::Text(s),
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
            Value::Bool(b) => Ok(Json::Bool(b)),
            Value::Integer(i) => {
                if let Some(i) = i.as_i64() {
                    return Ok(Json::Number(i.into()));
                }

                // Return value is definitely Some(T).
                Ok(Json::Number(
                    i.as_u64().expect("Invalid integer as JSON").into(),
                ))
            }
            Value::Float(f) => {
                let f = serde_json::Number::from_f64(f)
                    .with_context(|| format!("Infinite or NaN values are not allowed: {}", f))?;

                Ok(Json::Number(f))
            }
            Value::String(s) => Ok(Json::String(s)),
            Value::Array(arr) => {
                let arr: Result<Vec<_>> = arr.into_iter().map(|v| v.try_into()).collect();
                let arr = arr?;

                Ok(Json::Array(arr))
            }
            Value::Map(map) => {
                let values: Result<Vec<_>> = map.values().cloned().map(|v| v.try_into()).collect();
                let values = values?;

                Ok(Json::Object(
                    map.keys().cloned().zip(values.into_iter()).collect(),
                ))
            }
        }
    }
}

impl From<Value> for MessagePack {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => MessagePack::Nil,
            Value::Bool(b) => MessagePack::Boolean(b),
            Value::Integer(i) => {
                if let Some(i) = i.as_i64() {
                    return MessagePack::Integer(i.into());
                }

                // Return value is definitely Some(T).
                MessagePack::Integer(i.as_u64().expect("Invalid integer as MessagePack").into())
            }
            Value::Float(f) => MessagePack::F64(f),
            Value::String(s) => MessagePack::String(s.into()),
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
            Value::Bool(b) => Ok(Toml::Boolean(b)),
            Value::Integer(i) => {
                let i = i
                    .as_i64()
                    .with_context(|| format!("Out of range of integer: {}", i))?;

                Ok(Toml::Integer(i))
            }
            Value::Float(f) => Ok(Toml::Float(f)),
            Value::String(s) => {
                if let Ok(dt) = s.parse() {
                    return Ok(Toml::Datetime(dt));
                }

                Ok(Toml::String(s))
            }
            Value::Array(arr) => {
                let arr: Result<Vec<_>> = arr.into_iter().map(|v| v.try_into()).collect();
                let arr = arr?;

                Ok(Toml::Array(arr))
            }
            Value::Map(map) => {
                let values: Result<Vec<_>> = map.values().cloned().map(|v| v.try_into()).collect();
                let values = values?;

                Ok(Toml::Table(
                    map.keys().cloned().zip(values.into_iter()).collect(),
                ))
            }
        }
    }
}

impl From<Value> for Yaml {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Yaml::Null,
            Value::Bool(b) => Yaml::Bool(b),
            Value::Integer(i) => {
                if let Some(i) = i.as_i64() {
                    return Yaml::Number(i.into());
                }

                // Return value is definitely Some(T).
                Yaml::Number(i.as_u64().expect("Invalid integer as YAML").into())
            }
            Value::Float(f) => Yaml::Number(f.into()),
            Value::String(s) => Yaml::String(s),
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
