extern crate byteorder;
extern crate clap;
extern crate flate2;
extern crate hdf5;
extern crate math;
extern crate pretty_env_logger;

#[macro_use]
extern crate log;

mod csv;
mod eds;
mod h5;
mod mtx;
mod utils;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::io;
use utils::FileType;

fn convert_file(sub_m: &ArgMatches) -> Result<(), io::Error> {
    let input_file_path = sub_m.value_of("input").unwrap();

    let output_file_type = utils::find_output_format(sub_m);
    let (input_file_type, output_file_path) =
        utils::get_output_path(input_file_path, output_file_type.clone());

    let mut alphas: Vec<Vec<f32>> = Vec::new();
    let mut bit_vecs: Vec<Vec<u8>> = Vec::new();

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

    info!("Starting to read {} file", input_file_path);
    match input_file_type {
        FileType::EDS => eds::reader(
            input_file_path.clone(),
            num_cells,
            num_features,
            &mut alphas,
            &mut bit_vecs,
        )?,
        _ => unreachable!(),
    };

    info!("Done Reading Input file");
    info!("Output file path: {}", output_file_path);

    match output_file_type {
        FileType::MTX => mtx::writer(output_file_path, alphas, bit_vecs, num_cells, num_features)?,
        FileType::CSV => csv::writer(output_file_path, alphas, bit_vecs, num_cells, num_features)?,
        FileType::H5 => h5::writer(output_file_path, alphas, bit_vecs, num_cells, num_features)?,
        FileType::EDS => unreachable!(),
    };

    info!("All Done!");
    Ok(())
}

fn main() -> io::Result<()> {
    let matches = App::new("EDS")
        .version("0.1.0")
        .author("Avi Srivastava, Mike Love and Rob Patro")
        .about("Efficient scData Storage format")
        .subcommand(
            SubCommand::with_name("convert")
                .about("comnvert from eds data format to csv or mtx format")
                .arg(
                    Arg::with_name("mtx")
                        .long("mtx")
                        .conflicts_with("eds")
                        .conflicts_with("csv")
                        .conflicts_with("h5")
                        .help("convert to matrix market exchange file"),
                )
                .arg(
                    Arg::with_name("h5")
                        .long("h5")
                        .conflicts_with("eds")
                        .conflicts_with("csv")
                        .conflicts_with("mtx")
                        .help("convert to h5 wrapped csc file"),
                )
                .arg(
                    Arg::with_name("csv")
                        .long("csv")
                        .conflicts_with("eds")
                        .conflicts_with("mtx")
                        .conflicts_with("h5")
                        .help("convert to comma separated file"),
                )
                .arg(
                    Arg::with_name("eds")
                        .long("eds")
                        .conflicts_with("csv")
                        .conflicts_with("mtx")
                        .conflicts_with("h5")
                        .help("convert to EDS file"),
                )
                .arg(
                    Arg::with_name("cells")
                        .long("cells")
                        .short("c")
                        .takes_value(true)
                        .help("Number of cells"),
                )
                .arg(
                    Arg::with_name("features")
                        .long("features")
                        .short("f")
                        .takes_value(true)
                        .help("Number of features"),
                )
                .arg(
                    Arg::with_name("input")
                        .long("input")
                        .short("i")
                        .takes_value(true)
                        .requires("cells")
                        .requires("features")
                        .help("path to input file"),
                ),
        )
        .get_matches();

    pretty_env_logger::init_timed();
    match matches.subcommand_matches("convert") {
        Some(sub_m) => {
            let ret = convert_file(&sub_m);
            return ret;
        }
        None => (),
    };

    Ok(())
}
