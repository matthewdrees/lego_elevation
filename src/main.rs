use clap::Parser;
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
/// Supported latitude/longitude formats:
///
/// * "46° 51' 6" N 121° 45' 37" W"
/// * "N 46° 51' 6" W 121° 45' 37""
/// * "46° 51.1' N 121° 58.6167' W"
/// * "46.86167° N 121.76028° W"
/// * "46.86167 N 121.76028 W"
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Center of the map, latitude/longitude
    #[arg(short, long)]
    center: String,

    /// Map radius from the center, in kilometers
    #[arg(short, long)]
    radius: u16,

    /// Max level number
    #[arg(short, long)]
    levels: u8,

    #[arg(short, long)]
    /// Number of columns and rows
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

fn main() {
    let args = Args::parse();
    let center : geo::Point = latlon::parse(args.center).unwrap();
    let elevations = usgs::get_elevation_grid(&center, args.radius, args.gridsize);
    let lego_elevations : grid::Grid<u8> = get_lego_elevations(&elevations, args.levels);
    csv_out::write_grid_to_csv("elevation.csv", &lego_elevations);
}
