use std::fs::File;
use std::io;
use std::io::{Read, Write};

use byteorder::{ByteOrder, LittleEndian};
use flate2::read::GzDecoder;
use math::round;

use flate2::write::GzEncoder;
use flate2::Compression;

pub fn reader(
    input: &str,
    num_cells: usize,
    num_genes: usize,
    expr: &mut Vec<Vec<f32>>,
    bit_vecs: &mut Vec<Vec<u8>>,
) -> Result<bool, io::Error> {
    info!("Using {} as input EDS file\n", input);
    info!(
        "Using {} Rows (cells) and {} Columns (features)",
        num_cells, num_genes
    );

    let num_bit_vecs: usize = round::ceil(num_genes as f64 / 8.0, 0) as usize;
    let mut total_molecules = 0.0;
    let mut total_exp_values = 0;

    {
        let mut count = 0;
        let file_handle = File::open(input)?;
        let mut file = GzDecoder::new(file_handle);

        for _ in 0..num_cells {
            let mut bit_vec = vec![0; num_bit_vecs];
            file.read_exact(&mut bit_vec[..])?;
            let mut num_ones = 0;
            for bits in bit_vec.iter() {
                num_ones += bits.count_ones();
            }
            bit_vecs.push(bit_vec);

            let mut expression: Vec<u8> = vec![0; 4 * (num_ones as usize)];
            let mut float_buffer: Vec<f32> = vec![0.0_f32; num_ones as usize];
            file.read_exact(&mut expression[..])?;
            LittleEndian::read_f32_into(&expression, &mut float_buffer);

            let cell_count: f32 = float_buffer.iter().sum();
            total_molecules += cell_count;
            expr.push(float_buffer);

            count += 1;
            total_exp_values += num_ones;
            if count % 100 == 0 {
                print!("\r Done Reading {} cells", count);
                io::stdout().flush()?;
            }
        }
    }

    println!("\n");
    assert!(
        expr.len() == num_cells,
        "rows and quants file size mismatch"
    );

    info!("Found Total {:.2} molecules", total_molecules);
    info!("Found Total {:.2} expressed entries", total_exp_values);
    info!(
        "w/ {:.2} Molecules/cell",
        total_molecules / num_cells as f32
    );
    Ok(true)
}

pub fn writer(
    path_str: String,
    expressions: Vec<Vec<f32>>,
    bit_vecs: Vec<Vec<u8>>,
    _num_cells: usize,
    _num_features: usize,
) -> Result<bool, io::Error> {
    let file_handle = File::create(path_str)?;
    let mut file = GzEncoder::new(file_handle, Compression::default());

    assert!(expressions.len() == bit_vecs.len());
    for (exp, bvec) in expressions.into_iter().zip(bit_vecs.into_iter()) {
        file.write_all(&bvec)?;

        let mut bin_exp: Vec<u8> = vec![0_u8; exp.len() * 4];
        LittleEndian::write_f32_into(&exp, &mut bin_exp);
        file.write_all(&bin_exp)?;
    }

    Ok(true)
}
