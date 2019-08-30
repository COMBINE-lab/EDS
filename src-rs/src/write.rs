use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io;
use std::io::Write;
use std::mem;

use byteorder::ByteOrder;
use byteorder::LittleEndian;

pub fn write_mtx(
    input: &str,
    expressions: Vec<Vec<f32>>,
    bit_vecs: Vec<Vec<u8>>,
    num_cells: usize,
    num_features: usize,
) -> Result<bool, io::Error> {
    let mut path_str = input.to_string();
    let offset = path_str
        .find(".eds.gz")
        .unwrap_or(path_str.len());

    path_str.replace_range(offset.., ".mtx.gz");

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
    input: &str,
    expressions: Vec<Vec<f32>>,
    bit_vecs: Vec<Vec<u8>>,
    _num_cells: usize,
    num_features: usize,
) -> Result<bool, io::Error> {
    let mut path_str = input.to_string();
    let offset = path_str
        .find(".eds.gz")
        .unwrap_or(path_str.len());

    path_str.replace_range(offset.., ".csv.gz");

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

pub fn write_csc(
    input: &str,
    expressions: Vec<Vec<f32>>,
    bit_vecs: Vec<Vec<u8>>,
    num_cells: usize,
    _num_features: usize,
) -> Result<bool, io::Error> {
    let mut path_str = input.to_string();
    let offset = path_str
        .find(".eds.gz")
        .unwrap_or(path_str.len());

    path_str.replace_range(offset.., ".csc.gz");

    let file_handle = File::create(path_str)?;
    let mut file = GzEncoder::new(file_handle, Compression::default());

    assert!(bit_vecs.len() == expressions.len(), "length of bit vec and expression is not same");
    let mut lens: Vec<u32> = vec![0];
    for (index, exp) in expressions.iter().enumerate() {
        lens.push( lens[index] + exp.len() as u32 );
    }

    lens.pop();
    assert!(lens.len() == num_cells, "num cells doesn't match");
    let mut lens_bits: Vec<u8> = vec![0; mem::size_of::<u32>() * lens.len()];
    LittleEndian::write_u32_into(&lens, &mut lens_bits);
    file.write_all(&lens_bits)?;

    for (cell_id, bit_vec) in bit_vecs.iter().enumerate() {
        let exp = &expressions[cell_id];
        let mut expression: Vec<u8> = vec![0; mem::size_of::<f32>() * exp.len()];
        LittleEndian::write_f32_into(&exp, &mut expression);
        file.write_all(&expression)?;

        let mut fids: Vec<u32> = Vec::new();

        for (feature_id, flag) in bit_vec.into_iter().enumerate() {
            if *flag != 0 {
                for (offset, j) in format!("{:8b}", flag).chars().enumerate() {
                    match j {
                        '1' => fids.push( (8 * feature_id) as u32 + offset as u32 ),
                        _ => (),
                    };
                }
            }
        }

        assert!(fids.len() == expressions[cell_id].len(), "#positions doesn't match with #expressed features");
        let mut positions: Vec<u8> = vec![0; mem::size_of::<u32>() * fids.len()];
        LittleEndian::write_u32_into(&fids, &mut positions);
        file.write_all(&positions)?;
    }

    Ok(true)
}
