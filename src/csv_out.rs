// TODO make this generic
pub fn write_grid_to_csv(filename: &str, grid_vals: &grid::Grid<u8>) {
    let mut wtr = csv::Writer::from_path(filename).unwrap();
    for row_iter in grid_vals.iter_rows() {
        wtr.write_record(row_iter.map(|&x| x.to_string())).unwrap();
    }
    wtr.flush().unwrap();
}
