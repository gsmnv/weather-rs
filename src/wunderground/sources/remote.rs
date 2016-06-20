extern crate hyper;

use super::*;

use std::io::Read;
use std::error::Error;
use std::env;

pub struct Remote;

static API_BASE: &'static str = "http://api.wunderground.com/api/";

impl Source for Remote {
    fn hourly_forecast(&self) -> WeatherData {
        let mut s = String::new();
        let client = hyper::Client::new();
        let mut f = try!(client.get(try!(hourly_url()).as_str()).send());

        try!(f.read_to_string(&mut s));

        Ok(s)
    }

    fn current_conditions(&self) -> WeatherData {
        let mut s = String::new();
        let client = hyper::Client::new();
        let mut f = try!(client.get(try!(conditions_url()).as_str()).send());

        try!(f.read_to_string(&mut s));

        Ok(s)
    }
}

fn conditions_url() -> Result<String, Box<Error>> {
    build_url("/conditions")
}

fn hourly_url() -> Result<String, Box<Error>> {
    build_url("/hourly10day")
}

fn build_url(path: &str) -> Result<String, Box<Error>> {
    let token = try!(env::var("WUNDERGROUND_ACCESS_TOKEN"));
    let location = try!(env::var("WUNDERGROUND_LOCATION"));

    Ok(format!("{}{}{}/q/{}.json", API_BASE, token, path, location))
}
