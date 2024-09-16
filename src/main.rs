use minreq;
//use serde_json::{Value};
use serde::{Deserialize};

#[derive( Deserialize, Debug)]
struct NationalMapPointElevationResponse {
    value: f64, // elevation
}

fn get_elevation(x: f64, y: f64) -> i32 {
    let response = minreq::get("https://epqs.nationalmap.gov/v1/json")
        .with_param("x", x.to_string())
        .with_param("y", y.to_string())
        .with_param("wkid", "4326")
        .with_param("units", "feet")
        .with_header("accept", "application/json").send().unwrap_or_else(|error| {
            panic!("Response error: {error:?}");
        });
    assert_eq!(200, response.status_code);
    assert_eq!("OK", response.reason_phrase);
    let response_str = response.as_str().unwrap_or_else(|error| {
        panic!("Response string error: {error:?}");
    });
    println!("{response_str}");
    let nmper: NationalMapPointElevationResponse = serde_json::from_str(response_str).unwrap_or_else(|error| {
        panic!("Json response string error: {error:?}")
    });
    return nmper.value as i32;
}

// From here: https://gis.stackexchange.com/questions/251643/approx-distance-between-any-2-longitudes-at-a-given-latitude#:~:text=To%20convert%20a%20given%20latitude%20into%20the%20approximate,%2A%2069.172%20with%20the%20answer%20being%20~47%20miles.
// (90 - Decimal degrees) * Pi / 180 * 69.172

fn main() {
    let x = -121.760278;
    let y = 46.851667;
    let elevation = get_elevation(x, y);
    println!("x: {x}, y: {y}, elevation: {elevation}");
}
