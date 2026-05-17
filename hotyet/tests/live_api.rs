use hotyet::{Location, LocationWeather};

fn live_tests_enabled() -> bool {
    std::env::var("LIVE_API_TESTS").as_deref() == Ok("1")
}

#[test]
fn test_geocoding_city_response_live() {
    if !live_tests_enabled() {
        eprintln!("skipping live test; set LIVE_API_TESTS=1");
        return;
    }

    let location = Location::geocode_city("London");
    assert!(location.is_ok());
}

#[test]
fn test_weather_response_live() {
    if !live_tests_enabled() {
        eprintln!("skipping live test; set LIVE_API_TESTS=1");
        return;
    }

    let location = Location::geocode_city("London").expect("geocoding should succeed");
    let location_weather = LocationWeather::fetch_weather(location);
    assert!(location_weather.is_ok());
}
