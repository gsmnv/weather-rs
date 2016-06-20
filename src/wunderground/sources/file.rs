use super::*;

use std::fs::File;
use std::io::Read;

pub struct FileSource;

impl Source for FileSource {
    fn hourly_forecast(&self) -> WeatherData {
        let mut s = String::new();

        try!(try!(File::open("./tests/data/hourly.json")).read_to_string(&mut s));

        Ok(s)
    }

    fn current_conditions(&self) -> WeatherData {
        let mut s = String::new();

        try!(try!(File::open("./tests/data/current.json")).read_to_string(&mut s));

        Ok(s)
    }
}
