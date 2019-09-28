use std::io;
use std::thread;
use std::io::Write;
use std::cmp::Ordering::Equal;

use crate::csv;
use rayon::prelude::*;
use std::sync::mpsc::channel;

const K_NEAREST: usize = 10;

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

pub fn generate(bit_vecs: Vec<Vec<u8>>,
                alphas: Vec<Vec<f32>>,
) -> Result<(Vec<Vec<u8>>, Vec<Vec<f32>>), io::Error> {
    info!("generating Priors");
    assert!( bit_vecs.len() == alphas.len() );

    let num_cells = alphas.len();
    let num_genes = bit_vecs[0].len() * 8;

    let mut matrix: Vec<Vec<f32>> = vec![vec![0.0; num_genes]; num_cells];
    for i in 0..num_cells {
        let cell_bit_vec = &bit_vecs[i];
        let cell_alphas = &alphas[i];
        let mut aidx = 0;

        for (j, flag) in cell_bit_vec.into_iter().enumerate() {
            for (k, marker) in read_flag(*flag).into_iter().enumerate() {
                match marker {
                    true => {
                        let pos = j*8+k;
                        matrix[i][pos] = alphas[i][aidx];
                        aidx += 1;
                    },
                    false => (),
                }
            }
        }
        assert_eq!(aidx, cell_alphas.len());
    }
    info!("Done densifying matrix");

    // get all-v-all distances
    let (sender, receiver) = channel();
    rayon::ThreadPoolBuilder::new()
        .num_threads(20)
        .build_global()
        .unwrap();

    let mut dists = vec![vec![0.0; num_cells]; num_cells];
    let computation = thread::spawn(move || {
        let mut processed = 0;
        for value in receiver.iter() {
            match value {
                Some((i, j, dist)) => dists[i as usize][j] = dist,
                None => {
                    processed += 1;
                    print!("\r Done Processing {} cells.", processed);
                    io::stdout().flush().unwrap();
                },
            }// end-match
        }// end-for

        dists
    });

    matrix.par_iter()
        .enumerate()
        .for_each_with(sender, |s, (i, cell_1)| {
            for (j, cell_2) in matrix.iter().enumerate() {
                let mut dist = 0.0;
                cell_1.iter().zip(cell_2).for_each(|(a, b)| dist += (a - b).abs());
                s.send(Some((i, j, dist))).unwrap();
            }
            s.send(None).unwrap();
        });

    println!("\n");
    let dists = computation.join().unwrap();
    info!("Done All-v-All Distance Calculation");

    let mut ncounts = Vec::new();
    for (cell_id, cell_dists) in dists.into_iter().enumerate() {
        let mut cell_counts = matrix[cell_id].clone();
        let mut cell_dists_vec: Vec<_> = cell_dists.iter()
            .enumerate()
            .collect();

        cell_dists_vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Equal));
        for (nidx, (ncell_id, _)) in cell_dists_vec.into_iter().enumerate(){
            if nidx == 0 { continue; }

            cell_counts.iter_mut().zip(matrix[ncell_id].iter()).for_each(|(a, b)| *a += b);
            if nidx == K_NEAREST { break; } // use only top K_NEAREST
        }

        cell_counts.iter_mut()
            .for_each(|x| *x /= K_NEAREST as f32);

        ncounts.push(cell_counts);
    }
    info!("Done Creating Manhattan Prior");


    let (nbit_vecs, nalphas) = csv::dense_to_eds_sparse(ncounts, bit_vecs[0].len());
    Ok((nbit_vecs, nalphas))
}
