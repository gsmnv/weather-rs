use wunderground::deserializers::*;

use std::fmt::{ Display, Formatter, Result };

#[derive(Debug, Deserialize)]
pub struct CurrentConditions {
    #[serde(rename = "temp_c")]
    pub temperature: i8,

    #[serde(rename = "feelslike_c")]
    pub feels_like: i8,

    #[serde(rename = "wind_kph")]
    pub wind_speed: i8,

    #[serde(rename = "wind_dir")]
    pub wind_direction: String,

    #[serde(rename = "weather")]
    pub condition: String,

    #[serde(rename = "display_location", deserialize_with = "location")]
    pub location: String
}

impl Display for CurrentConditions {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}: {} ({})°C, {}, {} Wind {} km/h",
            self.location,
            self.temperature,
            self.feels_like,
            self.condition,
            self.wind_direction,
            self.wind_speed
        )
    }
}
