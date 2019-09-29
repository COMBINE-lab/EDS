use std::cmp::Ordering::Equal;
use std::io;
use std::io::Write;
use std::thread;

use crate::csv;
use rayon::prelude::*;
use std::sync::mpsc::channel;

const K_NEAREST: usize = 10;

fn get_all_v_all(
    num_cells: usize,
    matrix: &Vec<Vec<f32>>,
) -> Result<Vec<Vec<f32>>, std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>> {
    info!("Calculating All-v-All Distances");

    let num_threads = 20;
    let (sender, receiver) = channel();
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
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
                }
            } // end-match
        } // end-for

        dists
    });

    matrix
        .par_iter()
        .enumerate()
        .for_each_with(sender, |s, (i, cell_1)| {
            for (j, cell_2) in matrix.iter().enumerate() {
                let mut dist = 0.0;
                cell_1
                    .iter()
                    .zip(cell_2)
                    .for_each(|(a, b)| dist += (a - b).abs());
                s.send(Some((i, j, dist))).unwrap();
            }
            s.send(None).unwrap();
        });

    println!("\n");
    computation.join()
}

fn create_average_prior(
    matrix: Vec<Vec<f32>>,
    dists: Vec<Vec<f32>>,
) -> Result<Vec<Vec<f32>>, io::Error> {
    info!("Creating Averaging Prior");
    let mut priors = Vec::new();
    for (cell_id, cell_dists) in dists.into_iter().enumerate() {
        let mut cell_counts = matrix[cell_id].clone();
        let mut cell_dists_vec: Vec<_> = cell_dists.iter().enumerate().collect();

        cell_dists_vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Equal));
        for (nidx, (ncell_id, _)) in cell_dists_vec.into_iter().enumerate() {
            if nidx == 0 {
                continue;
            }

            cell_counts
                .iter_mut()
                .zip(matrix[ncell_id].iter())
                .for_each(|(a, b)| *a += b);
            if nidx == K_NEAREST {
                break;
            } // use only top K_NEAREST
        }

        cell_counts.iter_mut().for_each(|x| *x /= K_NEAREST as f32);

        priors.push(cell_counts);
    }

    Ok(priors)
}

fn create_cov_prior(
    num_cells: usize,
    num_genes: usize,
    matrix: Vec<Vec<f32>>,
    dists: Vec<Vec<f32>>,
) -> Result<Vec<Vec<f32>>, io::Error> {
    info!("Creating Intersection Prior");
    let mut priors = vec![vec![0.0; num_genes]; num_cells];
    for (cell_id, cell_dists) in dists.into_iter().enumerate() {
        // extract K Nearest cells
        let mut k_nearest_cells = Vec::new();
        {
            let mut cell_dists_vec: Vec<_> = cell_dists.into_iter().enumerate().collect();
            cell_dists_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Equal));
            for i in 0..K_NEAREST {
                k_nearest_cells.push(cell_dists_vec[i].0);
            }
        }

        let cell_prior = &mut priors[cell_id];
        for j in 0..cell_prior.len() {
            let mut count_distribution = Vec::new();
            for i in &k_nearest_cells {
                count_distribution.push( matrix[*i][j] )
            }

            let sum: f32 = count_distribution.iter().sum();
            let mean: f32 = sum / count_distribution.len() as f32;

            let mut variance: f32 = count_distribution.iter()
                .map(|x| (x-mean).powf(2.0))
                .sum();
            variance /= count_distribution.len() as f32;
            cell_prior[j] = mean / variance;
        }
    } // end-for cell-id for

    Ok(priors)
}

pub fn generate(
    bit_vecs: Vec<Vec<u8>>,
    alphas: Vec<Vec<f32>>,
) -> Result<(Vec<Vec<u8>>, Vec<Vec<f32>>), io::Error> {
    info!("generating Priors");
    assert!(bit_vecs.len() == alphas.len());

    let num_bvecs = bit_vecs[0].len();
    let num_genes = num_bvecs * 8;
    let num_cells = alphas.len();

    let matrix = csv::eds_sparse_to_dense(num_cells, num_genes, bit_vecs, alphas);

    let dists = get_all_v_all(num_cells, &matrix).expect("can't get all v all distances");

    let priors = create_cov_prior(num_cells, num_genes, matrix, dists)?;

    csv::dense_to_eds_sparse(priors, num_bvecs)
}

#[cfg(test)]
mod tests {
    use crate::prior::{generate, read_flag};

    #[test]
    fn test_read_flag() {
        assert_eq!(
            read_flag(128),
            vec![true, false, false, false, false, false, false, false]
        );
    }

    #[test]
    fn test_generate() {
        assert!(
            generate(vec![vec![0], vec![128]], vec![vec![], vec![22.0]]).unwrap()
                == (vec![vec![128], vec![128]], vec![vec![2.2], vec![2.2]])
        );
    }
}
