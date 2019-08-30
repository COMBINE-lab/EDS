use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io;
use std::io::Write;
use hdf5;

pub fn write_mtx(
    path_str: String,
    expressions: Vec<Vec<f32>>,
    bit_vecs: Vec<Vec<u8>>,
    num_cells: usize,
    num_features: usize,
) -> Result<bool, io::Error> {
    let mut tot_expressed_features = 0;
    expressions.iter()
        .for_each(|x| tot_expressed_features += x.len());

    let file_handle = File::create(path_str)?;
    let mut file = GzEncoder::new(file_handle, Compression::default());

    let mut header = "%%MatrixMarket\tmatrix\tcoordinate\treal\tgeneral\n".to_string();
    header.push_str(&format!("{}\t{}\t{}\n", num_cells, num_features, tot_expressed_features));
    file.write_all(header.as_bytes())?;

    assert!(bit_vecs.len() == expressions.len(), "length of bit vec and expression is not same");
    for (cell_id, exp) in expressions.into_iter().enumerate() {
        let bit_vec = &bit_vecs[cell_id];
        let mut fids: Vec<usize> = Vec::new();

        for (feature_id, flag) in bit_vec.into_iter().enumerate() {
            if *flag != 0 {
                for (offset, j) in format!("{:8b}", flag).chars().enumerate() {
                    match j {
                        '1' => fids.push( (8 * feature_id) + offset ),
                        _ => (),
                    };
                }
            }
        }

        assert!(fids.len() == exp.len(), "#positions doesn't match with #expressed features");
        let mut mtx_data = "".to_string();
        for (index, count) in exp.into_iter().enumerate() {
            mtx_data.push_str(&format!("{}\t{}\t{}\n", cell_id+1, fids[index] + 1, count));
        }

        file.write_all(mtx_data.as_bytes())?;
    }

    Ok(true)
}

pub fn write_csv(
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
    assert!(bit_vecs.len() == expressions.len(), "length of bit vec and expression is not same");
    for (cell_id, exp) in expressions.into_iter().enumerate() {
        let bit_vec = &bit_vecs[cell_id];
        let mut fids: Vec<usize> = Vec::new();

        for (feature_id, flag) in bit_vec.into_iter().enumerate() {
            if *flag != 0 {
                for (offset, j) in format!("{:8b}", flag).chars().enumerate() {
                    match j {
                        '1' => fids.push( (8 * feature_id) + offset ),
                        _ => (),
                    };
                }
            }
        }

        assert!(fids.len() == exp.len(), "#positions doesn't match with #expressed features");
        mtx_data = format!("cell{}", cell_id + 1);
        let mut zero_counter = 0;
        for (index, count) in exp.into_iter().enumerate() {
            assert!(fids[index] < num_features,
                    format!("{} position > {}", fids[index], num_features));

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

pub fn write_h5(
    path_str: String,
    expressions: Vec<Vec<f32>>,
    bit_vecs: Vec<Vec<u8>>,
    num_cells: usize,
    num_features: usize,
) -> Result<bool, io::Error> {
    let file = hdf5::File::open(path_str, "w")
        .expect("can't create output file");

    let group = file.create_group("matrix")
        .expect("can't create group in h5");

    let shape = group.new_dataset::<u64>()
        .create("shape", 2)
        .expect("can't write shape in h5");

    shape.write( &[num_features, num_cells] )
        .expect("error writing shape");

    assert!(bit_vecs.len() == expressions.len(), "length of bit vec and expression is not same");

    let total_entries;
    {
        let mut cumm_sum = 0;
        let mut indptr_vals: Vec<u32> = vec![ cumm_sum ];
        for exp in expressions.iter() {
            cumm_sum += exp.len() as u32;
            indptr_vals.push( cumm_sum );
        }

        total_entries = indptr_vals.last()
            .expect("indptr empty")
            .clone();
        assert!(indptr_vals.len() == num_cells + 1, "num cells doesn't match");

        let indptr = group.new_dataset::<u32>()
            .create("indptr", indptr_vals.len())
            .expect("can't write indptr in h5");

        indptr.write_raw( &indptr_vals )
            .expect("error writing indptr");

    } // end writing indptr

    {
        let data = group.new_dataset::<f32>()
            .create("data", total_entries as usize)
            .expect("can't write data in h5");

        let flatten_data: Vec<f32> = expressions.iter()
            .flat_map(|array| array.iter())
            .cloned()
            .collect();

        assert!(flatten_data.len() == total_entries as usize, "different number of entries");
        data.write_raw( &flatten_data )
            .expect("can't write data");

    } // end writing data

    {
        let indices = group.new_dataset::<u32>()
            .create("indices", total_entries as usize)
            .expect("can't write positions in h5");

        let mut positions: Vec<u32> = Vec::new();
        for bit_vec in bit_vecs {
            for (feature_id, flag) in bit_vec.into_iter().enumerate() {
                if flag != 0 {
                    for (offset, j) in format!("{:8b}", flag).chars().enumerate() {
                        match j {
                            '1' => positions.push( (8 * feature_id) as u32 + offset as u32 ),
                            _ => (),
                        };
                    }
                }
            }
        } // end-for

        assert!(positions.len() == total_entries as usize, "different number of entries");
        indices.write_raw( &positions )
            .expect("can't write indices");

    } // end writing indices

    Ok(true)
}
