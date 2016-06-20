use std::collections::BTreeMap;

use serde_json::Value;
use serde::{ Deserializer, Error };
use serde::de::impls::BTreeMapVisitor;

use chrono::DateTime;
use chrono::Local;
use chrono::TimeZone;

fn map_nested_field<T, D, F>(nested_field: &str, deserializer: &mut D, f: F) ->
    Result<T, D::Error>
    where D: Deserializer,
          F: Fn(String) -> Result<T, D::Error> {

    let visitor = BTreeMapVisitor::new();
    let hash: BTreeMap<String, Value> = try!(deserializer.deserialize(visitor));

    let value_err = Error::custom("Missing nested field");
    let string_err = Error::custom("Failed to convert field to string");

    let value = try!(hash.get(nested_field).ok_or(value_err));
    let string = try!(value.as_str().ok_or(string_err)).to_string();

    f(string)
}

pub fn wind_direction<D>(deserializer: &mut D) -> Result<String, D::Error>
    where D: Deserializer {

    map_nested_field("dir", deserializer, |value| Ok(value))
}

pub fn metric<D>(deserializer: &mut D) -> Result<i8, D::Error>
    where D: Deserializer {

    map_nested_field("metric", deserializer, |value| {
        value.as_str().parse().map_err(|_| {
            Error::custom("Failed to parse metric")
        })
    })
}

pub fn time<D>(deserializer: &mut D) -> Result<DateTime<Local>, D::Error>
    where D: Deserializer {

    map_nested_field("epoch", deserializer, |value| {
        let epoch = try!(value.as_str().parse().map_err(|_| {
            Error::custom("Failed to parse epoch")
        }));

        Ok(Local.timestamp(epoch, 0))
    })
}

pub fn location<D>(deserializer: &mut D) -> Result<String, D::Error>
    where D: Deserializer {

    map_nested_field("full", deserializer, |value| Ok(value))
}
