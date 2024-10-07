use anyhow::{Context, Result};
use geo;
use grid;
use minreq;
use log;
use serde::Deserialize;

const KILOMETERS_PER_LAT_DEGREE : f64 = 110.567;

/// For deserizliaing the JSON repsonse from USGS Point Query Service
#[derive(Deserialize, Debug)]
struct NationalMapPointElevationResponse {
    value: String, // elevation in meters
}

/// Fetch elevation from latitude/longitude using the USGS Point Query Service
///
/// https://apps.nationalmap.gov/epqs/
///
fn get_elevation_usgs_point_query_service(lat: f64, lon: f64) -> Result<i32> {
    let url = "https://epqs.nationalmap.gov/v1/json";
    let response = minreq::get(url)
        .with_param("x", lon.to_string())
        .with_param("y", lat.to_string())
        .with_param("wkid", "4326")
        .with_param("units", "meters")
        .with_header("accept", "application/json").send().with_context(||format!("error getting response from {url}"))?;
    if response.status_code != 200 {
        let status_code = response.status_code;
        let reason_phrase = response.reason_phrase;
        return Err(anyhow::anyhow!(format!("Bad http status {status_code}, reason: {reason_phrase}, from {url}")));
    }
    let response_str = response.as_str().with_context(||"http response string error")?;
    let nmper: NationalMapPointElevationResponse = serde_json::from_str(response_str).with_context(||format!("Json response string error from {url}. Text: '{response_str}'. Note: elevation data only supported in Canada, Mexico, and USA."))?;
    let elevation = nmper.value.parse::<f64>().with_context(||"meters string to f64 error")? as i32;
    Ok(elevation)
}

pub fn get_km_between_longitude_lines(lat : f64) -> f64 {
    assert!(lat < 90.0);
    // From here: https://gis.stackexchange.com/questions/251643/approx-distance-between-any-2-longitudes-at-a-given-latitude
    return (90.0 - lat.abs()) * std::f64::consts::PI / 180.0 * KILOMETERS_PER_LAT_DEGREE;
}

fn latlon_to_string(lat : f64, lon: f64) -> String {
    let latdir = if lat < 0.0 {"S"} else {"N"};
    let londir = if lon < 0.0 {"W"} else {"E"};
    let abslat = lat.abs();
    let abslon = lon.abs();
    return format!("{abslat:.5} {latdir}, {abslon:.6} {londir}");
}

fn validate_latitude(lat: f64) -> Result<()> {
    if lat > 90.0 || lat < -90.0 {
        return Err(anyhow::anyhow!("Bad latitude {lat}"));
    }
    Ok(())
}
fn validate_longitude(lon: f64) -> Result<()> {
    if lon > 180.0 || lon < -180.0 {
        return Err(anyhow::anyhow!("Bad longitude {lon}"));
    }
    Ok(())
}

pub fn get_elevation_grid<F: Fn()>(center: geo::Point, radius: u16, gridsize: i16, progress_update_func : F) -> Result<grid::Grid<i32>> {

    let grid_dim = gridsize as usize;
    let mut elevations : grid::Grid<i32> = grid::Grid::new(grid_dim, grid_dim);
    let mid = gridsize / 2;
    let f_radius = radius as f64;
    for y in 0..gridsize {
        let lat = center.y() + f_radius / KILOMETERS_PER_LAT_DEGREE * (mid - y) as f64 / mid as f64;
        validate_latitude(lat)?;
        let km_between_lons = get_km_between_longitude_lines(lat);
        for x in 0..gridsize {
            let lon = center.x() + f_radius / km_between_lons * (x - mid) as f64 / mid as f64;
            validate_longitude(lon)?;
            let elevation = get_elevation_usgs_point_query_service(lat, lon)?;
            elevations[(y as usize, x as usize)] = elevation;
            let latlonstr = latlon_to_string(lat, lon);
            log::info!("y: {y}, x: {x}, {latlonstr}, {elevation} meters");
            progress_update_func();
        }
    }

    Ok(elevations)
}
