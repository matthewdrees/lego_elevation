use clap;
use clap::{Parser};
use geo;
use grid;

mod csv_out;
mod usgs;

/// Fetch elevation data suitable for building a 3D relief map out of legos.
///
/// Example: (Mount Rainier)
///
///   $ lego_elevation --center "46°51′6″N 121°45′37″W" --radius 7 --levels 9 --gridsize 32
///
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true, version, about, long_about)]
struct Args {
    /// Center of the map, latitude/longitude. Supported formats:
    ///   "46° 51' 6" N 121° 45' 37" W"
    ///   "N 46° 51' 6" W 121° 45' 37""
    ///   "46° 51.1' N 121° 58.6167' W"
    ///   "46.86167° N 121.76028° W"
    ///   "46.86167 N 121.76028 W"
    #[arg(short, long, value_parser = parse_center)]
    center: geo::Point,

    /// Map radius from the center, in kilometers
    #[arg(short, long, value_parser= clap::value_parser!(u16).range(1..=10_000))]
    radius: u16,

    /// Number of levels
    #[arg(short, long, value_parser= clap::value_parser!(u8).range(1..=255))]
    levels: u8,

    /// Number of columns and rows
    #[arg(short, long, value_parser= clap::value_parser!(i16).range(1..=1000))]
    gridsize: i16,
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

fn parse_center(s: &str) -> Result<geo::Point, String> {
    return latlon::parse(s).map_err(|error| error.to_string());
}

fn main() {
    let args = Args::parse();
    let elevations = usgs::get_elevation_grid(args.center, args.radius, args.gridsize);
    let lego_elevations : grid::Grid<u8> = get_lego_elevations(&elevations, args.levels);
    csv_out::write_grid_to_csv("elevation.csv", &lego_elevations);
}
