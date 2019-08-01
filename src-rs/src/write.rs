use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn write_mtx(
    input: &str,
    expressions: Vec<Vec<f32>>,
    bit_vecs: Vec<Vec<u8>>,
) -> Result<bool, io::Error> {
    let mut path_str = input.to_string();
    let offset = path_str
        .find("mtx.gz")
        .unwrap_or(path_str.len());

    path_str.replace_range(offset.., "eds.gz");

    let file_handle = File::create(path_str)?;
    let mut file = GzEncoder::new(file_handle, Compression::default());
    let header = "%%MatrixMarket\tmatrix\tcoordinate\treal\tgeneral";
    file.write_all(header.as_bytes())?;

    assert!(bit_vecs.len() == expressions.len(), "length of bit vec and expression is not same");
    for (cell_id, exp) in expressions.into_iter().enumerate() {
        let bit_vec = &bit_vecs[cell_id];
        let mut fids: Vec<usize> = Vec::new();

        for (feature_id, flag) in bit_vec.into_iter().enumerate() {
            if *flag != 0 {
                for (offset, j) in format!("{:b}", flag).chars().enumerate() {
                    match j {
                        '1' => fids.push( (8 * feature_id) + offset ),
                        '0' => (),
                        _ => unreachable!(),
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
