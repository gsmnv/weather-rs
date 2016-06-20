extern crate serde_json;

mod forecasts;
mod deserializers;
pub mod sources;

use self::forecasts::*;
use self::sources::Source;

use serde_json::Value;
use std::error::Error;

pub struct Wunderground {
    source: Box<Source>
}

impl Wunderground {
    pub fn new(source: Option<Box<Source>>) -> Self {
        Wunderground { source: source.unwrap_or(Box::new(sources::Remote)) }
    }

    pub fn current_conditions(&self) -> Result<CurrentConditions, Box<Error>> {
        let data = try!(self.source.current_conditions());

        let value: Value = try!(serde_json::from_str(data.as_str()));
        let data_err = "Missing 'current_observation' JSON field";
        let obj = try!(value.as_object().ok_or("Value is not an object"));
        let data = try!(obj.get("current_observation").ok_or(data_err));
        let conditions = try!(serde_json::value::from_value(data.clone()));

        Ok(conditions)
    }

    pub fn hourly_forecast(&self) -> Result<HourlyForecast, Box<Error>> {
        let data = try!(self.source.hourly_forecast());
        let hourly = try!(serde_json::from_str(data.as_str()));

        Ok(hourly)
    }
}
