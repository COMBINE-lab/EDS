use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::fs::File;
use std::io;
use std::collections::HashMap;
use std::io::{Write, BufReader, BufRead};
use math::round;

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
    let file = BufReader::new( GzDecoder::new(file_handle) );

    let cell_index = 0;
    let gene_index = 1;
    let mut found_first = false;
    let mut triplets: Vec<HashMap<u32, f32>> = vec![ HashMap::new(); num_cells ];

    for line in file.lines() {
        let record = line?;
        if record.chars().nth(0).unwrap() == '%' {
            continue;
        }

        let vals: Vec<&str> = record.split("\t")
            .collect();

        let gid = vals[gene_index].parse::<u32>().unwrap();
        let cid = vals[cell_index].parse::<usize>().unwrap();
        let value = vals[2].parse::<f32>().unwrap();

        if ! found_first {
            found_first = true;

            assert!(num_cells == cid );
            assert!(num_genes == gid as usize);
            continue;
        }

        triplets[cid - 1].insert(gid - 1, value);
    }

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
