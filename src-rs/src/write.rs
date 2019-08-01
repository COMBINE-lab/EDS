use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn write_mtx(
    input: &str,
    expressions: Vec<Vec<f32>>,
    bit_vecs: Vec<Vec<u8>>,
    num_cells: usize,
    num_features: usize,
) -> Result<bool, io::Error> {
    let mut path_str = input.to_string();
    let offset = path_str
        .find("eds.gz")
        .unwrap_or(path_str.len());

    path_str.replace_range(offset.., "mtx.gz");

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
