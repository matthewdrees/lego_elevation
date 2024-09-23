// TODO: try using ETOPO Global Relief Model ...
//
//     https://www.ncei.noaa.gov/products/etopo-global-relief-model
//
// ... once I figure out how to open a geotiff file ...
//
//     https://docs.rs/tiff/latest/tiff/
//
// .. the geotiff files don't seem to be supported in v0.9.1.
// use tiff;
// use tiff::decoder::{Decoder, DecodingResult};
// use tiff::ColorType;
// use std::fs::File;

// pub fn get_elevation() -> i32 {
//     let img_file = File::open("ETOPO_2022_v1_15s_N60W135_geoid.tif").expect("Cannot find image");
//     let mut decoder = tiff::decoder::Decoder::new(img_file).expect("Cannot create decoder");
//     // decoder = decoder.with_limits(tiff::decoder::Limits::unlimited());
//     // let  dims = decoder.dimensions().expect("Cannot get dimensions");
//     // println!("dims = {:?}", dims);
//     // let color_type = decoder.colortype().expect("cannot get colortype");
//     // println!("color type {:?}", color_type);
//     // let tc = decoder.tile_count().expect("cannot get tile count");
//     // println!("tc: {tc}");
//     let DecodingResult::I16(data) = decoder.read_image().unwrap() else {
//         panic!("Cannot read band data")
//     };
//     let data_len = data.len();
//     // println!("data.len = {data_len}");
//     return data_len as i32;
// }
