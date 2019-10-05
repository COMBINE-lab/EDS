use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::fs::File;
use std::io;
use std::io::Write;
use std::collections::HashMap;
use other_csv;
use crate::utils;

fn read_flag(flag: u8) -> Vec<bool> {
    let mut markers: Vec<bool> = Vec::new();
    for j in format!("{:8b}", flag).chars() {
        match j {
            '1' => markers.push(true),
            _ => markers.push(false),
        };
    } // end-for

    markers
}

pub fn eds_sparse_to_dense(
    num_cells: usize,
    num_genes: usize,
    bit_vecs: Vec<Vec<u8>>,
    alphas: Vec<Vec<f32>>,
) -> Vec<Vec<f32>> {
    info!("Densifying matrix");
    let mut matrix: Vec<Vec<f32>> = vec![vec![0.0; num_genes]; num_cells];
    for i in 0..num_cells {
        let cell_bit_vec = &bit_vecs[i];
        let cell_alphas = &alphas[i];
        let mut aidx = 0;

        for (j, flag) in cell_bit_vec.into_iter().enumerate() {
            for (k, marker) in read_flag(*flag).into_iter().enumerate() {
                match marker {
                    true => {
                        let pos = j * 8 + k;
                        matrix[i][pos] = alphas[i][aidx];
                        aidx += 1;
                    }
                    false => (),
                }
            }
        }
        assert_eq!(aidx, cell_alphas.len());
    }
    matrix
}

pub fn dense_to_eds_sparse(
    matrix: Vec<Vec<f32>>,
    num_bvecs: usize,
) -> Result<(Vec<Vec<u8>>, Vec<Vec<f32>>), io::Error> {
    info!("Converting Dense matrix to EDS sparse");

    let num_cells = matrix.len();
    let mut bvecs = vec![vec![0; num_bvecs]; num_cells];
    let mut alphas = vec![Vec::new(); num_cells];
    for (row_id, row) in matrix.into_iter().enumerate() {
        assert!(row.len() == num_bvecs * 8);

        for (flag_id, flag) in bvecs[row_id].iter_mut().enumerate() {
            for i in 0..8 {
                let col_id = flag_id * 8 + i;
                if row[col_id] > 0.0 {
                    alphas[row_id].push(row[col_id]);
                    *flag |= 128u8 >> i;
                }
            }
        }
    }

    Ok((bvecs, alphas))
}

pub fn reader(
    input: &str,
    num_cells: usize,
    num_genes: usize,
    expr: &mut Vec<Vec<f32>>,
    bit_vecs: &mut Vec<Vec<u8>>,
) -> Result<bool, io::Error> {
    info!("Using {} as input CSV file\n", input);
    info!(
        "Using {} Rows (cells) and {} Columns (features)",
        num_cells, num_genes
    );

    let file_handle = File::open(input)?;
    let file = other_csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(GzDecoder::new(file_handle));

    let mut cid = 0;
    let mut triplets: Vec<HashMap<u32, f32>> = vec![HashMap::new(); num_cells];
    for line in file.into_records() {
        let vals: Vec<f32> = line.unwrap()
            .deserialize(None)
            .unwrap();

        for (gid, val) in vals.into_iter().enumerate() {
            triplets[cid].insert(gid as u32, val);
        }
        cid += 1;
    }
    assert!(num_cells == cid, format!("{}/{}", num_cells, cid));

    utils::triplets_to_eds(&triplets, expr, bit_vecs, num_genes);
    Ok(true)
}

pub fn writer(
    path_str: String,
    expressions: Vec<Vec<f32>>,
    bit_vecs: Vec<Vec<u8>>,
    _num_cells: usize,
    num_features: usize,
) -> Result<bool, io::Error> {
    let file_handle = File::create(path_str)?;
    let mut file = GzEncoder::new(file_handle, Compression::default());

    let mut header = "\"\"".to_string();
    for gid in 1..num_features + 1 {
        header.push_str(&format!(",gene{}", gid));
    }
    header.push_str(&format!("\n"));
    //file.write_all(header.as_bytes())?;

    let mut mtx_data: String;
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
            format!(
                "#positions {} doesn't match with #expressed features {}",
                fids.len(),
                exp.len()
            )
        );
        //mtx_data = format!("cell{}", cell_id + 1);
        mtx_data = format!("");
        let mut zero_counter = 0;
        for (index, count) in exp.into_iter().enumerate() {
            assert!(
                fids[index] < num_features,
                format!("{} position > {}", fids[index], num_features)
            );

            while zero_counter != fids[index] {
                zero_counter += 1;
                mtx_data.push_str(&format!("0,"));
            }

            zero_counter += 1;
            mtx_data.push_str(&format!("{},", count));
        }

        while zero_counter < num_features {
            zero_counter += 1;
            mtx_data.push_str(&format!("0,"));
        }

        mtx_data.push_str(&format!("\n"));
        file.write_all(mtx_data.as_bytes())?;
    }

    Ok(true)
}
