extern crate byteorder;
extern crate flate2;
extern crate hdf5;
extern crate math;
extern crate pretty_env_logger;
extern crate rayon;

#[macro_use]
extern crate log;

mod csv;
mod eds;
mod h5;
mod mtx;
mod prior;
mod utils;

use std::io;
use utils::FileType;
use clap::ArgMatches;

pub fn randomize_file(sub_m: &ArgMatches) -> Result<(), io::Error> {
    let input_file_path = sub_m.value_of("input").unwrap();
    let output_file_type = FileType::Dummy(".random".to_string());

    let (input_file_type, output_file_path) =
        utils::get_output_path(input_file_path, output_file_type.clone());

    let num_cells: usize = sub_m
        .value_of("cells")
        .expect("can't find #cells")
        .parse()
        .unwrap();

    let num_features = sub_m
        .value_of("features")
        .expect("can't find #features")
        .parse()
        .unwrap();

    let (bit_vecs, alphas) = utils::read_file(
        input_file_path,
        input_file_type.clone(),
        num_cells,
        num_features,
    )?;

    let (bit_vecs, alphas) = utils::randomize(bit_vecs, alphas)?;
    utils::write_file(
        output_file_path,
        output_file_type,
        bit_vecs,
        alphas,
        num_cells,
        num_features,
    )?;

    info!("All Done!");
    Ok(())
}

pub fn generate_prior(sub_m: &ArgMatches) -> Result<(), io::Error> {
    let input_file_path = sub_m.value_of("input").unwrap();
    let output_file_type = FileType::CSV;

    let (input_file_type, output_file_path) =
        utils::get_output_path(input_file_path, output_file_type.clone());

    let num_cells: usize = sub_m
        .value_of("cells")
        .expect("can't find #cells")
        .parse()
        .unwrap();

    let num_features = sub_m
        .value_of("features")
        .expect("can't find #features")
        .parse()
        .unwrap();

    let (bit_vecs, alphas) = utils::read_file(
        input_file_path,
        input_file_type.clone(),
        num_cells,
        num_features,
    )?;

    let (bit_vecs, alphas) = prior::generate(bit_vecs, alphas)?;
    utils::write_file(
        output_file_path,
        output_file_type,
        bit_vecs,
        alphas,
        num_cells,
        num_features,
    )?;

    info!("All Done!");
    Ok(())
}

pub fn convert_file(sub_m: &ArgMatches) -> Result<(), io::Error> {
    let input_file_path = sub_m.value_of("input").unwrap();
    let output_file_type = utils::find_output_format(sub_m);

    let (input_file_type, output_file_path) =
        utils::get_output_path(input_file_path, output_file_type.clone());

    let num_cells: usize = sub_m
        .value_of("cells")
        .expect("can't find #cells")
        .parse()
        .unwrap();

    let num_features = sub_m
        .value_of("features")
        .expect("can't find #features")
        .parse()
        .unwrap();

    let (bit_vecs, alphas) =
        utils::read_file(input_file_path, input_file_type, num_cells, num_features)?;

    utils::write_file(
        output_file_path,
        output_file_type,
        bit_vecs,
        alphas,
        num_cells,
        num_features,
    )?;

    info!("All Done!");
    Ok(())
}
