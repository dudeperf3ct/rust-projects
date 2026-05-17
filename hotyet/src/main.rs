// Weather application - Hotyet
// Workflow
// Input: City, temp format (F, K or C)
// Output: Temperature and If hot (tC > 26)
// Advanced: TUI app (new project)

// Learnings:
// Always propogate errors ?
// Where possible return Return<T, E> from libraries
// Custom error handling and type aliases
// Not happy with Temperature (or how best it could be handled)
// Biggest: serde_json and serde for serialization
// It can ignore the fields not present in the struct
// The request and response could be made more idomatic
// TODO: Async request handling

// Quirky: Bombay and Mumbai produced different results
// Current temperature at Bombay is Celsius(17.7)
// Current temperature at Mumbai is Celsius(30.9)

use std::{error::Error, io, process::exit};

use hotyet::{Location, LocationWeather};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Weather application");
    println!("What is name of the city?");
    let mut city = String::new();
    io::stdin().read_line(&mut city)?;
    let city = city.trim();
    if city.is_empty() {
        eprintln!("Expected the name of city");
        exit(1)
    }
    // dbg!(&city);
    let location = Location::geocode_city(city)?;
    let weather = LocationWeather::fetch_weather(location)?;
    println!(
        "Current temperature at {} is {:?}",
        weather.location.city, weather.temperature
    );
    println!("Is it hot? {}", weather.temperature.is_hot());
    Ok(())
}
