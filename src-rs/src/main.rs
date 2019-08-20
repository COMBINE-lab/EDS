extern crate byteorder;
extern crate clap;
extern crate flate2;
extern crate math;
extern crate pretty_env_logger;

#[macro_use]
extern crate log;

mod parse;
mod write;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::io;

fn convert_file(sub_m: &ArgMatches) -> Result<(), io::Error> {
    let file_path = sub_m.value_of("eds")
        .unwrap();

    let file_type = match sub_m.is_present("mtx") {
        true => "mtx",
        false => {
            match sub_m.is_present("csv") {
                true => "csv",
                false => panic!("Neither csv nor mtx output format specified"),
            }
        }
    };

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

    info!("Starting to read EDS file");
    parse::read_eds(
        file_path.clone(),
        num_cells,
        num_features,
        &mut alphas,
        &mut bit_vecs,
    )?;

    info!("Done Reading Quants; generating {}", file_type);
    match file_type {
        "mtx" => {
            write::write_mtx(file_path, alphas, bit_vecs, num_cells, num_features)?;
        }
        "csv" => {
            write::write_csv(file_path, alphas, bit_vecs, num_cells, num_features)?;
        }
        _ => unreachable!(),
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
                        .conflicts_with("csv")
                        .help("convert to mtx file"),
                )
                .arg(
                    Arg::with_name("csv")
                        .long("csv")
                        .conflicts_with("mtx")
                        .help("convert to csv file"),
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
                    Arg::with_name("eds")
                        .long("eds")
                        .short("e")
                        .takes_value(true)
                        .requires("cells")
                        .requires("features")
                        .help("path to (zipped) eds file"),
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
