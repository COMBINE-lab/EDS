use byteorder::{ByteOrder, LittleEndian};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn dump_data(
    input: &str,
    bit_vecs: (Vec<Vec<u16>>, Vec<u8>),
    expressions: Vec<Vec<f32>>,
) -> Result<bool, io::Error> {
    let path = Path::new(input);

    {
        let file_path = path.join("alevin").join("expressions.gz");
        let file_handle = File::create(file_path)?;
        let mut file = GzEncoder::new(file_handle, Compression::default());

        for expr in expressions {
            let mut bin_expr = vec![0; 4 * expr.len()];
            LittleEndian::write_f32_into(&expr, &mut bin_expr);
            file.write_all(&bin_expr)?;
        }
    }

    {
        let file_path = path.join("alevin").join("bit_vecs.gz");
        let file_handle = File::create(file_path)?;
        let mut file = GzEncoder::new(file_handle, Compression::default());

        file.write(&bit_vecs.1)?;
        let mut edits = 0;
        for bvec in bit_vecs.0 {
            edits += bvec.len();

            let mut bin_bvec: Vec<u8> = vec![0; 2 * bvec.len()];
            LittleEndian::write_u16_into(&bvec, &mut bin_bvec);

            let mut lens: Vec<u8> = vec![0; 2];
            LittleEndian::write_u16_into(&[bvec.len() as u16], &mut lens);

            file.write_all(&lens)?;
            file.write_all(&bin_bvec)?;
        }

        info!("Total Edits Found: {}", edits);
    }

    Ok(true)
}
