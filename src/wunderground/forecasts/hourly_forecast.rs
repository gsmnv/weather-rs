use wunderground::deserializers::*;

use chrono::DateTime;
use chrono::Local;

use std::fmt::Write;

use linked_hash_map::LinkedHashMap;

#[derive(Debug, Deserialize)]
pub struct HourForecast {
    #[serde(rename = "temp", deserialize_with = "metric")]
    pub temperature: i8,

    #[serde(rename = "feelslike", deserialize_with = "metric")]
    pub feels_like: i8,

    #[serde(rename = "FCTTIME", deserialize_with = "time")]
    pub time: DateTime<Local>,

    #[serde(rename = "wspd", deserialize_with = "metric")]
    pub wind_speed: i8,

    #[serde(rename = "wdir", deserialize_with = "wind_direction")]
    pub wind_direction: String,

    #[serde(rename = "pop")]
    pub precipitation_chance: i8,

    pub condition: String
}

#[derive(Debug, Deserialize)]
pub struct HourlyForecast {
    hourly_forecast: Vec<HourForecast>
}

impl HourlyForecast {
    pub fn days(&self, mut n: u8) -> String {
        let days = group_by_day(&self.hourly_forecast);
        let mut result = String::new();
        let mut days_iter = days.iter().peekable();

        if n == 0 {
            return result;
        }

        while let Some((date, hours)) = days_iter.next() {
            let mut hours_iter = hours.iter().peekable();

            writeln!(&mut result, "{}", date).unwrap();

            while let Some(hour) = hours_iter.next() {
                write_hour(&mut result, &hour);

                // Don't insert trailing newline after the last hour
                // in the last day.
                if hours_iter.peek().is_some() || (n > 1 && days_iter.peek().is_some()) {
                    result.push_str("\n");
                }
            }

            // Don't insert trailing newline after the last day.
            if days_iter.peek().is_some() && n != 1 {
                result.push_str("\n");
            }

            if n == 1 {
                return result;
            }

            n -= 1;
        }

        result
    }
}

fn write_hour(dest: &mut String, hour: &HourForecast) {
    write!(
        dest,
        "{}, T: {} ({})°C, {} ({}%), W: {} {} km/h",
        hour.time.format("%H:%M"),
        hour.temperature,
        hour.feels_like,
        hour.condition,
        hour.precipitation_chance,
        hour.wind_direction,
        hour.wind_speed
    ).unwrap();
}

fn group_by_day(hourly_forecast: &Vec<HourForecast>) ->
    LinkedHashMap<String, Vec<&HourForecast>>
{
    let mut days = LinkedHashMap::new();

    for hour in hourly_forecast {
        let date = hour.time.format("%A, %d %B %Y").to_string();

        if days.contains_key(&date) {
            let mut hours : &mut Vec<_> = days.get_mut(&date).unwrap();
            hours.push(hour);
        } else {
            days.insert(date, vec![hour]);
        }

    }

    days
}
