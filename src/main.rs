use clap::Parser;
use geo;
use grid;

mod csv_out;
mod elevation;

/// Supported formats for latitude/longitude. (Note put "double quotes" around it.)
///
/// * "40° 26' 46" N 79° 58' 56" W"
/// * "N 40° 26' 46" W 79° 58' 56""
/// * "40° 26.767' N 79° 58.933' W"
/// * "N 40° 26.767' W 79° 58.933'"
/// * "N 40.446° W 79.982°"
/// * "40.446° N 79.982° W"
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// latitude/longitude for the center of the map
    #[arg(short, long)]
    center: String,

    /// map radius from the center, in miles
    #[arg(short, long)]
    radius: u16,

    /// max level number
    #[arg(short, long)]
    levels: u8,

    #[arg(short, long)]
    /// number of column/row spacings
    spacings: i16,
}

fn get_lego_elevations(elevations: &grid::Grid<i32>, levels : u8) -> grid::Grid<u8> {

    let mut lego_elevations: grid::Grid<u8> = grid::Grid::new(elevations.rows(), elevations.cols());
    let min_elevation = elevations.iter().min().unwrap();
    let max_elevation = elevations.iter().max().unwrap() + 1;
    for (e, le) in elevations.iter().zip(lego_elevations.iter_mut()) {
        *le = ((e - min_elevation) * (levels as i32 + 1) / (max_elevation - min_elevation)) as u8;
    }
    return lego_elevations;
}

fn main() {
    let args = Args::parse();
    let center : geo::Point = latlon::parse(args.center).unwrap();

    // TODO: center pass in f64,f64 for lat/lon.
    // TODO: CLI bounds checking.

    // Mt Rainier
    // let center = geo::Point::new(-121.760278, 46.851667);

    // Mt Kilimanjaro
    // let center = geo::Point::new(37.35333333,-3.075833333);

    let elevations = elevation::get_elevation_grid(&center, args.radius, args.spacings);
    println!("elevations: {elevations:?}");
    let lego_elevations : grid::Grid<u8> = get_lego_elevations(&elevations, args.levels);
    csv_out::write_grid_to_csv("elevation.csv", &lego_elevations);
}
