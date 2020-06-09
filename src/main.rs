extern crate clap;
mod weather;

use std::fmt::{self, Display};
use std::{process};

use weather::{get_info, QueryType, WeatherInfo};

use serde::Deserialize;

use clap::{App, Arg};

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub api_key: String,
    pub city_id: String,
    pub units: String,
    pub lang: String,
    pub display_symbol: String,
    pub enable_forcast: String
}

fn get_forecast(api_key: &str, city_id: &str, lang: &str, units: &str, display_symbol: &str, enable_forcast: &str) -> Result<String, Error> {
    let config = Configuration {
        api_key: api_key.to_string(),
        city_id: city_id.to_string(),
        lang: lang.to_string(),
        units: units.to_string(),
        display_symbol: display_symbol.to_string(),
        enable_forcast: enable_forcast.to_string()
    };

    let c = get_info(&config, QueryType::Current)?;
    let f = get_info(&config, QueryType::Forecast)?;

    if c.temperature < f.temperature {
        Ok(format_output(c, f, &config.enable_forcast, &config.display_symbol, "%{T3}%{T-}"))
    } else if c.temperature > f.temperature {
        Ok(format_output(c, f, &config.enable_forcast, &config.display_symbol, "%{T3}%{T-}"))
    } else {
        Ok(format_output(c, f, &config.enable_forcast, &config.display_symbol, "%{T3}%{T-}"))
    }
}

fn format_output(current: WeatherInfo, forecast: WeatherInfo, enable_forcast: &str, unit: &str, trend: &str) -> String {
 
    let prefix = format!(
        "%{{T4}}{ci}%{{T-}} %{{T1}}{ct}{u}%{{T-}}",
        ct = current.temperature,
        ci = current.icon,
        u = unit
    );
 
    let suffix = format!(
        "%{{T4}}{fi}%{{T-}} %{{T1}}{ft}{u}%{{T-}}",
        ft = forecast.temperature,
        fi = forecast.icon,
        u = unit
    );

   if enable_forcast == "true" {
        return [prefix, suffix].join(trend);
    } else {
        return prefix;
    }
}

fn main() {
    let matches = App::new("Polybar Weather Forcast")
        .version("1.0")
        .author("Aakash Gajjar <skyqutip@gmail.com>")
        .about("Display weather info in Polybar")
        .arg(
            Arg::with_name("api-key")
                .short("k")
                .long("api-key")
                .required(true)
                .takes_value(true)
                .help("Open Weather API key"),
        )
        .arg(
            Arg::with_name("city-id")
                .short("c")
                .long("city-id")
                .required(true)
                .takes_value(true)
                .help("City ID from openweather.com"),
        )
        .arg(
            Arg::with_name("units")
                .short("u")
                .long("units")
                .required(false)
                .takes_value(true)
                .default_value("kelvin")
                .possible_values(&["kelvin", "metric", "imperial"])
                .help("Unit of temperature")
        )
        .arg(
            Arg::with_name("lang")
                .short("l")
                .long("language")
                .required(false)
                .takes_value(true)
                .default_value("en")
                .help("Localization language (when description is visible)")
        )
        .arg(
            Arg::with_name("enable-forcast")
                .short("f")
                .long("enable-forcast")
                .required(false)
                .takes_value(true)
                .default_value("false")
                .help("Display forcast for next day")
                .possible_values(&["true", "false"])
        )
        .get_matches();

    let api_key = matches.value_of("api-key").unwrap();
    let city_id = matches.value_of("city-id").unwrap();
    let units = matches.value_of("units").unwrap();
    let lang = matches.value_of("lang").unwrap();
    let enable_forcast = matches.value_of("enable-forcast").unwrap();
    let mut display_symbol = "K";

   if units == "metric" {
        display_symbol = "°C";
    } else if units == "imperial" {
        display_symbol = "°F";
    }
    
    match get_forecast(api_key, city_id, lang, units, display_symbol, enable_forcast) {
        Ok(forecast) => println!("{}", forecast),
        Err(e) => {
            // Line break prevents massive errors from trashing the bar,
            // Polybar displays everything until the first line break
            eprintln!("\nForecast unavailable ({})", e);
            process::exit(1);
        }
    }
    println!("{} {} {} {} {} {}", api_key, city_id, units, lang, display_symbol, enable_forcast);
}

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    MissingConfigFile(std::io::Error),
    InvalidConfigFile(toml::de::Error),
    InvalidResponse,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            HttpError(e) => write!(f, "Failed to query OpenWeatherMap: {:?}", e),
            MissingConfigFile(e) => write!(f, "Could not find config file: {:?}", e),
            InvalidConfigFile(e) => write!(f, "Could not parse config file as TOML: {:?}", e),
            InvalidResponse => write!(f, "Invalid response format from OpenWeatherMap"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::HttpError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::InvalidConfigFile(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::MissingConfigFile(err)
    }
}
