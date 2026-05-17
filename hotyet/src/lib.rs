use serde::{Deserialize, Serialize};

pub type WeatherResult<T> = Result<T, WeatherError>;

#[derive(Debug)]
pub enum WeatherError {
    Request(reqwest::Error),
    LocationNotFound(String),
    MissingTemperature,
    Api(String),
}

impl From<reqwest::Error> for WeatherError {
    fn from(error: reqwest::Error) -> Self {
        WeatherError::Request(error)
    }
}

impl std::fmt::Display for WeatherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeatherError::Request(err) => write!(f, "Request failed: {err}"),
            WeatherError::LocationNotFound(city) => write!(f, "Location not found: {city}"),
            WeatherError::MissingTemperature => write!(f, "Missing temperature in response"),
            WeatherError::Api(message) => write!(f, "API error: {message}"),
        }
    }
}

impl std::error::Error for WeatherError {}

// const OPENSTREET_MAP_URL: &str = "https://nominatim.openstreetmap.org/search?q=";
const OPEN_METEO_GEOCODE_URL: &str = "https://geocoding-api.open-meteo.com/v1/search?name=";
const OPEN_METEO_WEATHER_URL: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Debug, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

impl Temperature {
    pub fn as_celsius(&self) -> f64 {
        match self {
            Temperature::Celsius(value) => *value,
            Temperature::Fahrenheit(value) => (*value - 32.0) * 5.0 / 9.0,
            Temperature::Kelvin(value) => *value - 273.15,
        }
    }

    pub fn is_hot(&self) -> bool {
        self.as_celsius() > 26.
    }

    pub fn as_fahrenheit(&self) -> f64 {
        self.as_celsius() * 9.0 / 5.0 + 32.0
    }

    pub fn as_kelvin(&self) -> f64 {
        self.as_celsius() + 273.15
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename(deserialize = "name"))]
    pub city: String,
    country: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoCodeResponse {
    results: Vec<Location>,
}

impl Location {
    pub fn geocode_city(city: &str) -> WeatherResult<Location> {
        println!("Fetching latitude and longitude for city:{city}");
        let url = format!(
            "{url}{city}&count={count}&language={language}&format={format}",
            url = OPEN_METEO_GEOCODE_URL,
            city = city,
            count = 1,
            language = "en",
            format = "json"
        );
        // dbg!(&url);
        // TODO: Async version
        // let client = reqwest::Client::new();
        // let contents = client.get(url).send().await?.json().await?;
        let response: GeoCodeResponse = reqwest::blocking::get(url)?
            .error_for_status()?
            .json::<GeoCodeResponse>()?;
        // dbg!(&contents);

        response
            .results
            .into_iter()
            .next()
            .ok_or_else(|| WeatherError::LocationNotFound(city.to_string()))
    }
}

#[derive(Debug)]
pub struct LocationWeather {
    pub location: Location,
    pub temperature: Temperature,
}

#[derive(Debug, serde::Deserialize)]
struct WeatherResponse {
    current: CurrentWeather,
}

#[derive(Debug, serde::Deserialize)]
struct CurrentWeather {
    temperature_2m: Option<f64>,
}

impl LocationWeather {
    pub fn fetch_weather(location: Location) -> WeatherResult<LocationWeather> {
        println!("Fetching weather details for city:{}", location.city);
        let url = format!(
            "{url}?latitude={latitude}&longitude={longitude}&current={current}",
            url = OPEN_METEO_WEATHER_URL,
            latitude = location.latitude,
            longitude = location.longitude,
            current = "temperature_2m"
        );
        // dbg!(&url);
        // TODO: Aync version
        // TODO: Handle missing temperature : Option<f64> in response
        let response: WeatherResponse = reqwest::blocking::get(url)?.error_for_status()?.json()?;
        // dbg!(&response.current);

        let temperature_celsius = response
            .current
            .temperature_2m
            .ok_or(WeatherError::MissingTemperature)?;

        Ok(LocationWeather {
            location,
            temperature: Temperature::Celsius(temperature_celsius),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_celsius_to_fahrenheit() {
        let temp_celsius = Temperature::Celsius(25.0);
        let fahrenheit = temp_celsius.as_fahrenheit();
        assert_eq!(77.0, fahrenheit)
    }

    #[test]
    fn test_convert_celsius_to_kelvin() {
        let temp_celsius = Temperature::Celsius(25.0);
        let kelvin = temp_celsius.as_kelvin();
        assert_eq!(298.15, kelvin)
    }
}
