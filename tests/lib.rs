extern crate weather;

use weather::wunderground::*;

static HOURLY_10_DAYS_EXPECTATION: &'static str =
    r#"Tuesday, 03 July 2012
22:00, T: 19 (19)°C, Clear (0%), W: West 8 km/h
23:00, T: 19 (19)°C, Clear (0%), W: West 14 km/h

Wednesday, 04 July 2012
00:00, T: 20 (20)°C, Clear (0%), W: West 21 km/h
01:00, T: 20 (20)°C, Clear (0%), W: WSW 27 km/h
02:00, T: 20 (20)°C, Clear (0%), W: WSW 27 km/h
03:00, T: 20 (20)°C, Clear (0%), W: WSW 27 km/h
04:00, T: 20 (20)°C, Clear (0%), W: WSW 27 km/h
05:00, T: 20 (20)°C, Clear (0%), W: WSW 24 km/h
06:00, T: 19 (19)°C, Clear (0%), W: WSW 21 km/h
07:00, T: 19 (19)°C, Clear (0%), W: WSW 18 km/h
08:00, T: 17 (17)°C, Clear (0%), W: WSW 17 km/h
09:00, T: 16 (16)°C, Clear (0%), W: WSW 15 km/h
10:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 14 km/h
11:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 14 km/h
12:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 14 km/h
13:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 14 km/h
14:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 14 km/h
15:00, T: 13 (13)°C, Mostly Cloudy (0%), W: SSW 13 km/h
16:00, T: 13 (13)°C, Mostly Cloudy (0%), W: SSW 13 km/h
17:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 13 km/h
18:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 13 km/h
19:00, T: 15 (15)°C, Mostly Cloudy (0%), W: SSW 13 km/h
20:00, T: 17 (17)°C, Mostly Cloudy (0%), W: SSW 15 km/h
21:00, T: 20 (20)°C, Mostly Cloudy (0%), W: SSW 17 km/h
22:00, T: 22 (22)°C, Clear (0%), W: SW 19 km/h
23:00, T: 21 (21)°C, Clear (0%), W: SW 21 km/h

Thursday, 05 July 2012
00:00, T: 21 (21)°C, Clear (0%), W: SW 22 km/h
01:00, T: 20 (20)°C, Clear (0%), W: WSW 24 km/h
02:00, T: 20 (20)°C, Clear (0%), W: WSW 22 km/h
03:00, T: 19 (19)°C, Clear (0%), W: WSW 21 km/h
04:00, T: 19 (19)°C, Clear (0%), W: WSW 19 km/h
05:00, T: 18 (18)°C, Clear (0%), W: WSW 19 km/h
06:00, T: 18 (18)°C, Clear (0%), W: WSW 19 km/h
07:00, T: 17 (17)°C, Clear (0%), W: WSW 19 km/h
08:00, T: 16 (16)°C, Clear (0%), W: WSW 19 km/h
09:00, T: 15 (15)°C, Clear (0%), W: WSW 18 km/h"#;

static HOURLY_2_DAYS_EXPECTATION: &'static str =
    r#"Tuesday, 03 July 2012
22:00, T: 19 (19)°C, Clear (0%), W: West 8 km/h
23:00, T: 19 (19)°C, Clear (0%), W: West 14 km/h

Wednesday, 04 July 2012
00:00, T: 20 (20)°C, Clear (0%), W: West 21 km/h
01:00, T: 20 (20)°C, Clear (0%), W: WSW 27 km/h
02:00, T: 20 (20)°C, Clear (0%), W: WSW 27 km/h
03:00, T: 20 (20)°C, Clear (0%), W: WSW 27 km/h
04:00, T: 20 (20)°C, Clear (0%), W: WSW 27 km/h
05:00, T: 20 (20)°C, Clear (0%), W: WSW 24 km/h
06:00, T: 19 (19)°C, Clear (0%), W: WSW 21 km/h
07:00, T: 19 (19)°C, Clear (0%), W: WSW 18 km/h
08:00, T: 17 (17)°C, Clear (0%), W: WSW 17 km/h
09:00, T: 16 (16)°C, Clear (0%), W: WSW 15 km/h
10:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 14 km/h
11:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 14 km/h
12:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 14 km/h
13:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 14 km/h
14:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 14 km/h
15:00, T: 13 (13)°C, Mostly Cloudy (0%), W: SSW 13 km/h
16:00, T: 13 (13)°C, Mostly Cloudy (0%), W: SSW 13 km/h
17:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 13 km/h
18:00, T: 14 (14)°C, Mostly Cloudy (0%), W: SSW 13 km/h
19:00, T: 15 (15)°C, Mostly Cloudy (0%), W: SSW 13 km/h
20:00, T: 17 (17)°C, Mostly Cloudy (0%), W: SSW 15 km/h
21:00, T: 20 (20)°C, Mostly Cloudy (0%), W: SSW 17 km/h
22:00, T: 22 (22)°C, Clear (0%), W: SW 19 km/h
23:00, T: 21 (21)°C, Clear (0%), W: SW 21 km/h"#;

static CURRENT_EXPECTATION: &'static str =
    r#"Pulkovo, Russia: 15 (15)°C, Clear, West Wind 14 km/h"#;

#[test]
fn hourly_forecast_10_days_rendering() {
    let wunderground = Wunderground::new(Some(Box::new(sources::FileSource)));

    assert_eq!(
        HOURLY_10_DAYS_EXPECTATION,
        format!("{}", wunderground.hourly_forecast().unwrap().days(10))
    );
}

#[test]
fn hourly_forecast_2_days_rendering() {
    let wunderground = Wunderground::new(Some(Box::new(sources::FileSource)));

    assert_eq!(
        HOURLY_2_DAYS_EXPECTATION,
        format!("{}", wunderground.hourly_forecast().unwrap().days(2))
    );
}

#[test]
fn currrent_forecast_rendering() {
    let wunderground = Wunderground::new(Some(Box::new(sources::FileSource)));

    assert_eq!(
        CURRENT_EXPECTATION,
        format!("{}", wunderground.current_conditions().unwrap())
    );
}
