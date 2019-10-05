use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

use crate::utils::triplets_to_eds;
pub fn reader(
    input: &str,
    num_cells: usize,
    num_genes: usize,
    expr: &mut Vec<Vec<f32>>,
    bit_vecs: &mut Vec<Vec<u8>>,
) -> Result<bool, io::Error> {
    info!("Using {} as input MTX file\n", input);
    info!(
        "Using {} Rows (cells) and {} Columns (features)",
        num_cells, num_genes
    );

    let file_handle = File::open(input)?;
    let file = BufReader::new(GzDecoder::new(file_handle));

    let cell_by_gene = true;
    let (cell_index, gene_index) = match cell_by_gene {
        true => (0, 1),
        false => (1, 0),
    };

    let mut found_first = false;
    let mut triplets: Vec<HashMap<u32, f32>> = vec![HashMap::new(); num_cells];

    for line in file.lines() {
        let record = line?;
        if record.chars().nth(0).unwrap() == '%' {
            continue;
        }

        let vals: Vec<&str> = record.split_whitespace().collect();

        let gid = vals[gene_index].parse::<u32>().expect("can't convert gid");
        let cid = vals[cell_index]
            .parse::<usize>()
            .expect("can't convert cid");
        let value = vals[2].parse::<f32>().expect("can't convert value");

        if !found_first {
            found_first = true;

            assert!(num_cells == cid);
            assert!(num_genes == gid as usize);
            continue;
        }

        triplets[cid - 1].insert(gid - 1, value);
    }

    triplets_to_eds(&triplets, expr, bit_vecs, num_genes);
    Ok(true)
}

pub fn writer(
    path_str: String,
    expressions: Vec<Vec<f32>>,
    bit_vecs: Vec<Vec<u8>>,
    num_cells: usize,
    num_features: usize,
) -> Result<bool, io::Error> {
    let mut tot_expressed_features = 0;
    expressions
        .iter()
        .for_each(|x| tot_expressed_features += x.len());

    let file_handle = File::create(path_str)?;
    let mut file = GzEncoder::new(file_handle, Compression::default());

    let mut header = "%%MatrixMarket\tmatrix\tcoordinate\treal\tgeneral\n".to_string();
    header.push_str(&format!(
        "{}\t{}\t{}\n",
        num_cells, num_features, tot_expressed_features
    ));
    file.write_all(header.as_bytes())?;

    assert!(
        bit_vecs.len() == expressions.len(),
        "length of bit vec and expression is not same"
    );
    for (cell_id, exp) in expressions.into_iter().enumerate() {
        let bit_vec = &bit_vecs[cell_id];
        let mut fids: Vec<usize> = Vec::new();

        for (feature_id, flag) in bit_vec.into_iter().enumerate() {
            if *flag != 0 {
                for (offset, j) in format!("{:8b}", flag).chars().enumerate() {
                    match j {
                        '1' => fids.push((8 * feature_id) + offset),
                        _ => (),
                    };
                }
            }
        }

        assert!(
            fids.len() == exp.len(),
            "#positions doesn't match with #expressed features"
        );
        let mut mtx_data = "".to_string();
        for (index, count) in exp.into_iter().enumerate() {
            mtx_data.push_str(&format!(
                "{}\t{}\t{}\n",
                cell_id + 1,
                fids[index] + 1,
                count
            ));
        }

        file.write_all(mtx_data.as_bytes())?;
    }

    Ok(true)
}
