extern crate clap;

use eds;
use std::io;
use clap::{App, Arg, SubCommand};

fn main() -> io::Result<()> {
    let matches = App::new("EDS")
        .version("0.1.0")
        .author("Avi Srivastava, Mike Love and Rob Patro")
        .about("Efficient scData Storage format")
        .subcommand(
            SubCommand::with_name("randomize")
                .about("randomize the order of cells")
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
        .subcommand(
            SubCommand::with_name("prior")
                .about("generate priors for alevin 2")
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
            let ret = eds::convert_file(&sub_m);
            return ret;
        }
        None => (),
    };

    match matches.subcommand_matches("randomize") {
        Some(sub_m) => {
            let ret = eds::randomize_file(&sub_m);
            return ret;
        }
        None => (),
    };

    match matches.subcommand_matches("prior") {
        Some(sub_m) => {
            let ret = eds::generate_prior(&sub_m);
            return ret;
        }
        None => (),
    };

    Ok(())
}
