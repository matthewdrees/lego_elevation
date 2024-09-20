use geo;
use grid;
use minreq;
use serde::{Deserialize};

#[derive( Deserialize, Debug)]
struct NationalMapPointElevationResponse {
    value: f64, // elevation
}
/// Fetch elevation from latitude/longitude using the USGS Point Query Service
///
/// https://apps.nationalmap.gov/epqs/
///
fn get_elevation_usgs_point_query_service(lat: f64, lon: f64) -> i32 {
    let response = minreq::get("https://epqs.nationalmap.gov/v1/json")
        .with_param("x", lon.to_string())
        .with_param("y", lat.to_string())
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
    // println!("{response_str}");
    let nmper: NationalMapPointElevationResponse = serde_json::from_str(response_str).unwrap_or_else(|error| {
        panic!("Json response string error: {error:?}")
    });
    return nmper.value as i32;
}

pub fn get_miles_between_longitude_lines(lat : f64) -> f64 {
    let absolute_lat = lat.abs();
    if absolute_lat >= 90.0 {
        panic!("bad absolute latitude {absolute_lat}");
    }
    // From here: https://gis.stackexchange.com/questions/251643/approx-distance-between-any-2-longitudes-at-a-given-latitude#:~:text=To%20convert%20a%20given%20latitude%20into%20the%20approximate,%2A%2069.172%20with%20the%20answer%20being%20~47%20miles.
    // (90 - Decimal degrees) * Pi / 180 * 69.172
    return (90.0 - absolute_lat) * std::f64::consts::PI / 180.0 * 69.172;
}

pub fn get_elevation_grid(center: &geo::Point, radius: u16, spacings: i16) -> grid::Grid<i32> {

    let mid = spacings / 2;
    let f_radius = radius as f64;
    let grid_dim = spacings as usize;
    let mut elevations : grid::Grid<i32> = grid::Grid::new(grid_dim, grid_dim);
    for y in 0..spacings {
        let lat = center.y() + f_radius / 69.172 * (mid - y) as f64 / mid as f64;
        let miles_between_lons = get_miles_between_longitude_lines(lat);
        for x in 0..spacings {
            let lon = center.x() + f_radius / miles_between_lons * (x - mid) as f64 / mid as f64;
            let elevation = get_elevation_usgs_point_query_service(lat, lon);
            println!("y: {y}, x: {x}, lat: {lat}, lon: {lon}, elevation: {elevation}");
            elevations[(y as usize, x as usize)] = elevation;
        }
    }
    return elevations;
}
