//
// SPDX-License-Identifier: Apache-2.0
//
// Copyright (C) 2021 Shun Sakai
//

use std::convert::{From, TryFrom, TryInto};
use std::str::FromStr;

use anyhow::{bail, Context, Result};
use serde_json::Value as Json;
use serde_yaml::Value as Yaml;
use toml::Value as Toml;

use crate::value::Value;

impl From<Json> for Value {
    fn from(value: Json) -> Self {
        match value {
            Json::Null => Value::Null,
            Json::Bool(b) => Value::Bool(b),
            Json::Number(n) => {
                if let Some(i) = n.as_i64() {
                    return Value::Int(i);
                }
                if let Some(u) = n.as_u64() {
                    return Value::UInt(u);
                }

                // Return value is definitely Some(T).
                Value::Float(n.as_f64().unwrap())
            }
            Json::String(s) => Value::String(s),
            Json::Array(arr) => Value::Array(arr.into_iter().map(|v| v.into()).collect()),
            Json::Object(obj) => Value::Map(obj.into_iter().map(|(k, v)| (k, v.into())).collect()),
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
                    return Ok(Value::Int(i));
                }
                if let Some(u) = n.as_u64() {
                    return Ok(Value::UInt(u));
                }

                // Return value is definitely Some(T).
                Ok(Value::Float(n.as_f64().unwrap()))
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
                    .into_iter()
                    .map(|k| {
                        k.as_str()
                            .context("The key is not a string.")
                            .map(|k| k.to_string())
                    })
                    .collect();
                let keys = keys?;
                let values: Result<Vec<_>> = values.into_iter().map(|v| v.try_into()).collect();
                let values = values?;
                let map = keys.into_iter().zip(values.into_iter()).collect();

                Ok(Value::Map(map))
            }
        }
    }
}

impl From<Toml> for Value {
    fn from(value: Toml) -> Self {
        match value {
            Toml::String(s) => Value::String(s),
            Toml::Integer(i) => Value::Int(i),
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

impl TryFrom<Value> for Json {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Null => Ok(Json::Null),
            Value::Bool(b) => Ok(Json::Bool(b)),
            Value::Int(i) => Ok(Json::Number(i.into())),
            Value::UInt(u) => Ok(Json::Number(u.into())),
            Value::Float(f) => {
                let f = serde_json::Number::from_f64(f).with_context(|| {
                    format!("Infinite or NaN values are not allowed in JSON: {}", f)
                })?;

                Ok(Json::Number(f))
            }
            Value::String(s) => Ok(Json::String(s)),
            Value::Array(arr) => {
                let arr: Result<Vec<_>> = arr.into_iter().map(|v| v.try_into()).collect();
                let arr = arr?;

                Ok(Json::Array(arr))
            }
            Value::Map(map) => {
                let (keys, values): (Vec<_>, Vec<_>) = map.into_iter().unzip();
                let values: Result<Vec<_>> = values.into_iter().map(|v| v.try_into()).collect();
                let values = values?;
                let obj = keys.into_iter().zip(values.into_iter()).collect();

                Ok(Json::Object(obj))
            }
        }
    }
}

impl From<Value> for Yaml {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Yaml::Null,
            Value::Bool(b) => Yaml::Bool(b),
            Value::Int(i) => Yaml::Number(i.into()),
            Value::UInt(u) => Yaml::Number(u.into()),
            Value::Float(f) => Yaml::Number(f.into()),
            Value::String(s) => Yaml::String(s),
            Value::Array(arr) => Yaml::Sequence(arr.into_iter().map(|v| v.into()).collect()),
            Value::Map(map) => {
                Yaml::Mapping(map.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
            }
        }
    }
}

impl TryFrom<Value> for Toml {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Null => bail!("Null is not allowed in TOML."),
            Value::Bool(b) => Ok(Toml::Boolean(b)),
            Value::Int(i) => Ok(Toml::Integer(i)),
            Value::UInt(u) => bail!("Out of range of integer of TOML: {}", u),
            Value::Float(f) => Ok(Toml::Float(f)),
            Value::String(s) => {
                if let Ok(dt) = toml::value::Datetime::from_str(&s) {
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
                let (keys, values): (Vec<_>, Vec<_>) = map.into_iter().unzip();
                let values: Result<Vec<_>> = values.into_iter().map(|v| v.try_into()).collect();
                let values = values?;
                let table = keys.into_iter().zip(values.into_iter()).collect();

                Ok(Toml::Table(table))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use once_cell::sync::Lazy;

    use super::*;

    // This sample data was imported from Wikidata.
    const JSON_SAMPLE_STRING: &str = indoc! {r#"
        {
          "name": "Joetsu Shinkansen",
          "operating_speed": 240,
          "line_length": 269.5,
          "is_owned_by_operator": true,
          "opened": "1982-11-15",
          "rolling_stock": [
            "E2 series",
            "E4 series",
            "E7 series"
          ],
          "stations": {
            "omiya": {
              "platforms": 6,
              "transfers": [
                "Kawagoe Line",
                "Keihin-Tohoku Line",
                "New Shuttle",
                "Saikyo Line",
                "Takasaki Line",
                "Tobu Urban Park Line",
                "Tohoku Main Line",
                "Tohoku Shinkansen",
                "Utsunomiya Line"
              ]
            },
            "niigata": {
              "platforms": 4,
              "transfers": [
                "Banetsu West Line",
                "Echigo Line",
                "Hakushin Line",
                "Shinetsu Main Line"
              ]
            }
          }
        }
    "#};
    const YAML_SAMPLE_STRING: &str = indoc! {r#"
        ---
        name: Joetsu Shinkansen
        operating_speed: 240
        line_length: 269.5
        is_owned_by_operator: true
        opened: 1982-11-15
        rolling_stock:
          - E2 series
          - E4 series
          - E7 series
        stations:
          omiya:
            platforms: 6
            transfers:
              - Kawagoe Line
              - Keihin-Tohoku Line
              - New Shuttle
              - Saikyo Line
              - Takasaki Line
              - Tobu Urban Park Line
              - Tohoku Main Line
              - Tohoku Shinkansen
              - Utsunomiya Line
          niigata:
            platforms: 4
            transfers:
              - Banetsu West Line
              - Echigo Line
              - Hakushin Line
              - Shinetsu Main Line
    "#};
    const TOML_SAMPLE_STRING: &str = indoc! {r#"
        name = 'Joetsu Shinkansen'
        operating_speed = 240
        line_length = 269.5
        is_owned_by_operator = true
        opened = 1982-11-15
        rolling_stock = [
            'E2 series',
            'E4 series',
            'E7 series',
        ]
        [stations.omiya]
        platforms = 6
        transfers = [
            'Kawagoe Line',
            'Keihin-Tohoku Line',
            'New Shuttle',
            'Saikyo Line',
            'Takasaki Line',
            'Tobu Urban Park Line',
            'Tohoku Main Line',
            'Tohoku Shinkansen',
            'Utsunomiya Line',
        ]

        [stations.niigata]
        platforms = 4
        transfers = [
            'Banetsu West Line',
            'Echigo Line',
            'Hakushin Line',
            'Shinetsu Main Line',
        ]
    "#};

    const JSON_SAMPLE_VALUE: Lazy<Json> =
        Lazy::new(|| serde_json::from_str(JSON_SAMPLE_STRING).unwrap());
    const YAML_SAMPLE_VALUE: Lazy<Yaml> =
        Lazy::new(|| serde_yaml::from_str(YAML_SAMPLE_STRING).unwrap());
    const TOML_SAMPLE_VALUE: Lazy<Toml> = Lazy::new(|| toml::from_str(TOML_SAMPLE_STRING).unwrap());

    #[test]
    fn json2yaml() {
        let ir_value: Value = Lazy::force(&JSON_SAMPLE_VALUE).clone().into();
        let converted: Yaml = ir_value.into();

        assert_eq!(converted, *YAML_SAMPLE_VALUE);
    }

    #[test]
    fn json2toml() {
        let ir_value: Value = Lazy::force(&JSON_SAMPLE_VALUE).clone().into();
        let converted: Toml = ir_value.try_into().unwrap();

        assert_eq!(converted, *TOML_SAMPLE_VALUE);
    }

    #[test]
    fn yaml2json() {
        let ir_value: Value = Lazy::force(&YAML_SAMPLE_VALUE).clone().try_into().unwrap();
        let converted: Json = ir_value.try_into().unwrap();

        assert_eq!(converted, *JSON_SAMPLE_VALUE);
    }

    #[test]
    fn yaml2toml() {
        let ir_value: Value = Lazy::force(&YAML_SAMPLE_VALUE).clone().try_into().unwrap();
        let converted: Toml = ir_value.try_into().unwrap();

        assert_eq!(converted, *TOML_SAMPLE_VALUE);
    }

    #[test]
    fn toml2json() {
        let ir_value: Value = Lazy::force(&TOML_SAMPLE_VALUE).clone().into();
        let converted: Json = ir_value.try_into().unwrap();

        assert_eq!(converted, *JSON_SAMPLE_VALUE);
    }

    #[test]
    fn toml2yaml() {
        let ir_value: Value = Lazy::force(&TOML_SAMPLE_VALUE).clone().into();
        let converted: Yaml = ir_value.into();

        assert_eq!(converted, *YAML_SAMPLE_VALUE);
    }

    #[test]
    fn non_string_yaml_key() {
        let yaml: Yaml = serde_yaml::from_str("256: byte").unwrap();
        let ir_value: Result<Value> = yaml.try_into();

        assert_eq!(
            ir_value.unwrap_err().to_string(),
            "The key is not a string."
        );
    }

    #[test]
    fn invalid_json_float() {
        let nan = Value::Float(f64::NAN);
        let invalid_value: Result<Json> = nan.try_into();

        assert_eq!(
            invalid_value.unwrap_err().to_string(),
            "Infinite or NaN values are not allowed in JSON: NaN"
        );
    }

    #[test]
    fn invalid_toml_null() {
        let null = Value::Null;
        let invalid_value: Result<Toml> = null.try_into();

        assert_eq!(
            invalid_value.unwrap_err().to_string(),
            "Null is not allowed in TOML."
        );
    }

    #[test]
    fn invalid_toml_integer() {
        let over_i64_max = Value::UInt(i64::MAX as u64 + 1);
        let invalid_value: Result<Toml> = over_i64_max.try_into();

        assert_eq!(
            invalid_value.unwrap_err().to_string(),
            "Out of range of integer of TOML: 9223372036854775808"
        );
    }
}
