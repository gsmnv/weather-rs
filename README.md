# Weather
Rust CLI weather forecasting tool.

## Installation
Builds only on Rust Nightly.
`cargo install weather`

## Supported forecast sources

### Wunderground
Requires `WUNDERGROUND_ACCESS_TOKEN` and `WUNDERGROUND_LOCATION` environment
variables set.
Sign up [here](https://www.wunderground.com/weather/api) to get you access
token. To find out your preferred location one could check [History](https://www.wunderground.com/history/airport/KSFO/2016/06/26/DailyHistory.html?req_city=San%20Francisco&req_state=CA&reqdb.zip=94101&reqdb.magic=1&reqdb.wmo=99999) 
tab on city page, in case of San Francisco, CA it would be KSFO.

#### Supported forcast types

##### Current conditions
Use `weather w c` to see current conditions at your station.

##### Hourly forecast
Use `weather w h` to see hourly forecast for next 10 days. Number of days to
display can be configured by passing `-d` parameter with integer value.
