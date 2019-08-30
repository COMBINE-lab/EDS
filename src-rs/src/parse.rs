use std::fs::File;
use std::io;
use std::io::{Read, Write};

use byteorder::{ByteOrder, LittleEndian};
use flate2::read::GzDecoder;
use math::round;

pub fn read_eds(
    input: &str,
    num_cells: usize,
    num_genes: usize,
    expr: &mut Vec<Vec<f32>>,
    bit_vecs: &mut Vec<Vec<u8>>,
) -> Result<bool, io::Error> {
    info!("Using {} as input EDS file\n", input);
    info!("Using {} Rows (cells) and {} Columns (features)", num_cells, num_genes);

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

//#[cfg(test)]
//mod tests {
//    use super::read_alevin_quants;
//
//    #[test]
//    fn neurons_900() {
//        let input_directory = "/mnt/scratch5/avi/alevin/bin/salmon/tests/alevin_test_data/";
//        let mut expr: Vec<Vec<f32>> = Vec::new();
//        let mut bit_vecs: Vec<Vec<u8>> = Vec::new();
//
//        match read_alevin_quants(input_directory, &mut expr, &mut bit_vecs) {
//            Ok(true) => (),
//            Ok(false) => panic!(),
//            Err(_) => panic!(),
//        };
//
//        let mut total_molecules = 0.0;
//        for cell in &expr {
//            let cell_count: f32 = cell.iter().sum();
//            total_molecules += cell_count;
//        }
//
//        assert!(total_molecules >= 3425370.0 && total_molecules <= 3425372.0);
//    }
//}
