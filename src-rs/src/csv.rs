use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn dense_to_eds_sparse(matrix: Vec<Vec<f32>>,
                           num_bvecs: usize,
) -> (Vec<Vec<u8>>, Vec<Vec<f32>>) {
    info!("Converting Dense matrix to EDS sparse");

    let num_cells = matrix.len();
    let mut bvecs = vec![vec![0; num_bvecs]; num_cells];
    let mut alphas = vec![Vec::new(); num_cells];
    for (row_id, row) in matrix.into_iter().enumerate() {
        assert!( row.len() == num_bvecs * 8 );

        for (flag_id, flag) in bvecs[row_id].iter_mut().enumerate(){
            for i in 0..8 {
                let col_id = flag_id*8+i;
                if row[col_id] > 0.0 {
                    alphas[row_id].push(row[col_id]);
                    *flag |= 128u8 >> i;
                }
            }
        }
    }

    (bvecs, alphas)
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
    file.write_all(header.as_bytes())?;

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
            format!("#positions {} doesn't match with #expressed features {}",
                    fids.len(), exp.len())
        );
        mtx_data = format!("cell{}", cell_id + 1);
        let mut zero_counter = 0;
        for (index, count) in exp.into_iter().enumerate() {
            assert!(
                fids[index] < num_features,
                format!("{} position > {}", fids[index], num_features)
            );

            while zero_counter != fids[index] {
                zero_counter += 1;
                mtx_data.push_str(&format!(",0"));
            }

            zero_counter += 1;
            mtx_data.push_str(&format!(",{}", count));
        }

        while zero_counter < num_features {
            zero_counter += 1;
            mtx_data.push_str(&format!(",0"));
        }

        mtx_data.push_str(&format!("\n"));
        file.write_all(mtx_data.as_bytes())?;
    }

    Ok(true)
}
