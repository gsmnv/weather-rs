mod remote;
mod file;

pub use self::remote::*;
pub use self::file::*;

use std::error::Error;

pub type WeatherData = Result<String, Box<Error>>;

pub trait Source {
    fn hourly_forecast(&self) -> WeatherData;
    fn current_conditions(&self) -> WeatherData;
}
