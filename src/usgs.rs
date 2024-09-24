use geo;
use grid;
use minreq;
use serde::{Deserialize};

const KILOMETERS_PER_LAT_DEGREE : f64 = 110.567;

#[derive( Deserialize, Debug)]
struct NationalMapPointElevationResponse {
    value: String, // elevation in meters
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
        .with_param("units", "meters")
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
    return nmper.value.parse::<f64>().unwrap_or_else(|error| {
        panic!("meters string to f64 error: {error:?}");
    }) as i32;
}

pub fn get_km_between_longitude_lines(lat : f64) -> f64 {
    let absolute_lat = lat.abs();
    if absolute_lat >= 90.0 {
        panic!("bad absolute latitude {absolute_lat}");
    }
    // From here: https://gis.stackexchange.com/questions/251643/approx-distance-between-any-2-longitudes-at-a-given-latitude
    return (90.0 - absolute_lat) * std::f64::consts::PI / 180.0 * KILOMETERS_PER_LAT_DEGREE;
}

fn latlon_to_string(lat : f64, lon: f64) -> String {
    let latdir = if lat < 0.0 {"S"} else {"N"};
    let londir = if lon < 0.0 {"W"} else {"E"};
    let abslat = lat.abs();
    let abslon = lon.abs();
    return format!("{abslat:.5} {latdir}, {abslon:.6} {londir}");
}

pub fn get_elevation_grid(center: geo::Point, radius: u16, gridsize: i16) -> grid::Grid<i32> {

    let mid = gridsize / 2;
    let f_radius = radius as f64;
    let grid_dim = gridsize as usize;
    let mut elevations : grid::Grid<i32> = grid::Grid::new(grid_dim, grid_dim);
    for y in 0..gridsize {
        let lat = center.y() + f_radius / KILOMETERS_PER_LAT_DEGREE * (mid - y) as f64 / mid as f64;
        let km_between_lons = get_km_between_longitude_lines(lat);
        for x in 0..gridsize {
            let lon = center.x() + f_radius / km_between_lons * (x - mid) as f64 / mid as f64;
            let elevation = get_elevation_usgs_point_query_service(lat, lon);
            let latlonstr = latlon_to_string(lat, lon);
            println!("y: {y}, x: {x}, \"{latlonstr}\", {elevation} meters");
            elevations[(y as usize, x as usize)] = elevation;
        }
    }
    return elevations;
}
