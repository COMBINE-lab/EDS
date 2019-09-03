use math::round;
use clap::ArgMatches;
use std::collections::HashMap;

use std;
use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::{h5, mtx, csv, eds};

#[derive(Clone, Debug, PartialEq)]
pub enum FileType {
    EDS,
    MTX,
    H5,
    CSV,
    Dummy(String),
}


pub fn write_file( file_path: String,
                   file_type: FileType,
                   bit_vecs: Vec<Vec<u8>>,
                   alphas: Vec<Vec<f32>>,
                   num_cells: usize,
                   num_features: usize,
) -> Result<bool, io::Error> {
    info!("Writing Output into file path: {}", file_path);

    match file_type {
        FileType::MTX => mtx::writer(file_path, alphas, bit_vecs, num_cells, num_features)?,
        FileType::CSV => csv::writer(file_path, alphas, bit_vecs, num_cells, num_features)?,
        FileType::H5 => h5::writer(file_path, alphas, bit_vecs, num_cells, num_features)?,
        FileType::EDS => eds::writer(file_path, alphas, bit_vecs, num_cells, num_features)?,
        _ => unreachable!(),
    };

    Ok(true)
}

pub fn read_file(file_path: &str,
                 file_type: FileType,
                 num_cells: usize,
                 num_features: usize,
) -> Result<(Vec<Vec<u8>>, Vec<Vec<f32>>), io::Error> {
    let mut alphas: Vec<Vec<f32>> = Vec::new();
    let mut bit_vecs: Vec<Vec<u8>> = Vec::new();

    match file_type {
        FileType::EDS => eds::reader(
            file_path,
            num_cells,
            num_features,
            &mut alphas,
            &mut bit_vecs,
        )?,
        FileType::MTX => mtx::reader(
            file_path,
            num_cells,
            num_features,
            &mut alphas,
            &mut bit_vecs,
        )?,
        _ => unreachable!(),
    };

    info!("Done Reading Input file");
    Ok((bit_vecs, alphas))
}

pub fn randomize(bit_vecs: Vec<Vec<u8>>,
                 alphas: Vec<Vec<f32>>,
) -> Result<(Vec<Vec<u8>>, Vec<Vec<f32>>), io::Error> {
    info!("Randomizing order");
    assert!( bit_vecs.len() == alphas.len() );

    let num_elem = bit_vecs.len() as u32;
    let mut order: Vec<u32> = (0..num_elem).collect();
    order.shuffle(&mut thread_rng());

    let mut shuf_bvecs = vec![Vec::new(); bit_vecs.len()];
    let mut shuf_alphas = vec![Vec::new(); bit_vecs.len()];
    for (nindex, oindex) in order.into_iter().enumerate() {
        shuf_bvecs[nindex] = bit_vecs[oindex as usize].clone();
        shuf_alphas[nindex] = alphas[oindex as usize].clone();
    }

    Ok((shuf_bvecs, shuf_alphas))
}

pub fn find_output_format(sub_m: &ArgMatches) -> FileType {
    let mut out_file_type: Option<FileType> = None;
    let mut found_file_types = 0;

    out_file_type = match sub_m.is_present("mtx") {
        true => {
            found_file_types += 1;
            Some(FileType::MTX)
        }
        false => out_file_type,
    };

    out_file_type = match sub_m.is_present("csv") {
        true => {
            found_file_types += 1;
            Some(FileType::CSV)
        }
        false => out_file_type,
    };

    out_file_type = match sub_m.is_present("h5") {
        true => {
            found_file_types += 1;
            Some(FileType::H5)
        }
        false => out_file_type,
    };

    out_file_type = match sub_m.is_present("eds") {
        true => {
            found_file_types += 1;
            Some(FileType::EDS)
        }
        false => out_file_type,
    };

    assert!(found_file_types == 1, "found unexpected not 1 file types");
    return out_file_type.expect("can't find output format type");
}

pub fn get_output_path(input_path: &str, otype: FileType) -> (FileType, String) {
    let mut itype: FileType = FileType::EDS;
    let mut opath = input_path.to_string();
    let mut offset: usize = opath.len();
    let mut found_file_types = 0;

    match opath.find(".eds") {
        Some(val) => {
            offset = val;
        }
        _ => (),
    };

    match opath.find(".mtx") {
        Some(val) => {
            offset = val;
            itype = FileType::MTX;
            found_file_types += 1;
        }
        _ => (),
    };

    match opath.find(".h5") {
        Some(val) => {
            offset = val;
            itype = FileType::H5;
            found_file_types += 1;
        }
        _ => (),
    };

    match opath.find(".csv") {
        Some(val) => {
            offset = val;
            itype = FileType::CSV;
            found_file_types += 1;
        }
        _ => (),
    };

    assert!(
        found_file_types == 1 || itype == FileType::EDS,
        " Can't find right input file type "
    );
    assert!(
        itype != otype,
        "Found same input and output file file format"
    );

    info!(" Found {:?} as input file type ", itype);
    info!(" Found {:?} as output file type ", otype);

    match otype {
        FileType::MTX => opath.replace_range(offset.., ".mtx.gz"),
        FileType::CSV => opath.replace_range(offset.., ".csv.gz"),
        FileType::H5 => opath.replace_range(offset.., ".h5"),
        FileType::EDS => opath.replace_range(offset.., ".eds.gz"),
        FileType::Dummy(name) => opath.replace_range(offset.., &name),
    }

    (itype, opath)
}

pub fn triplets_to_eds(triplets: &Vec<HashMap<u32, f32>>,
                       expr: &mut Vec<Vec<f32>>,
                       bit_vecs: &mut Vec<Vec<u8>>,
                       num_genes: usize,
) {
    for cell_data in triplets {
        let mut keys: Vec<u32> = cell_data.keys()
            .cloned()
            .collect();
        keys.sort();

        let values: Vec<f32> = keys.iter().map( |key| cell_data[key] )
            .collect();

        expr.push(values);

        let num_exp_genes = keys.len();
        let num_bit_vecs: usize = round::ceil(num_genes as f64 / 8.0, 0) as usize;
        let mut bit_vec: Vec<u8> = vec![0; num_bit_vecs];

        let mut min_processed_close = 0;
        let mut max_processed_open = 8;
        let mut curr_index = 0;
        let mut flag: u8 = 0;

        for key in keys {
            assert!(key >= min_processed_close);
            assert!(curr_index < num_bit_vecs);

            let offset: u8 = (key % 8) as u8;
            if key < max_processed_open {
                flag |= 128u8 >> offset;
            } else {
                bit_vec[curr_index] = flag;

                while key >= max_processed_open {
                    curr_index += 1;
                    min_processed_close = max_processed_open;
                    max_processed_open += 8;
                }
                flag = 128u8 >> offset;
            }
        }
        bit_vec[curr_index] = flag;

        let mut num_ones = 0;
        for bits in bit_vec.iter() {
            num_ones += bits.count_ones();
        }
        assert!(num_ones as usize == num_exp_genes,
                format!("{:?} {:?}", num_ones, num_exp_genes));

        bit_vecs.push(bit_vec);
    }
}
