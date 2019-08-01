extern crate byteorder;
extern crate clap;
extern crate flate2;
extern crate pretty_env_logger;
extern crate math;

#[macro_use]
extern crate log;

mod parse;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::io;

fn convert_file(sub_m: &ArgMatches) -> Result<(), io::Error> {
    let (file_type, file_path) = match sub_m.value_of("mtx") {
        Some(path) => ("mtx", path),
        None => {
            match sub_m.value_of("eds") {
                Some(path) => ("eds", path),
                None => unreachable!(),
            }
        },
    };

    match file_type {
        "eds" => {
            let mut alphas: Vec<Vec<f32>> = Vec::new();
            let mut bit_vecs: Vec<Vec<u8>> = Vec::new();

            info!("Starting to read EDS file");
            parse::read_eds(file_path, &mut alphas, &mut bit_vecs)?;

            info!("Done Reading Quants; generating mtx");
            //write::write_mtx(iput, mst, alphas)?;
        },
        "mtx" => {
            
        },
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
                .about("comnvert to/from eds data format")
                .arg(
                    Arg::with_name("mtx")
                        .long("mtx")
                        .short("m")
                        .takes_value(true)
                        .conflicts_with("eds")
                        .help("path to (zipped) mtx file"),
                )
                .arg(
                    Arg::with_name("eds")
                        .long("eds")
                        .short("e")
                        .takes_value(true)
                        .conflicts_with("mtx")
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
