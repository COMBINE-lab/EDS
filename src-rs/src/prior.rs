use std::io;
use std::collections::HashMap;
use std::cmp::Ordering::Equal;

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

fn get_manhattan_distance(bit_vecs: &Vec<Vec<u8>>,
                          alphas: &Vec<Vec<f32>>,
                          cid1: usize, cid2: usize)
                          -> Result<f32, io::Error> {
    let (bvec1, alphas1) = (&bit_vecs[cid1], &alphas[cid1]);
    let (bvec2, alphas2) = (&bit_vecs[cid2], &alphas[cid2]);

    assert_eq!(bvec1.len(), bvec2.len());
    let mut distance: f32 = 0.0;
    let (mut spos1, mut spos2) = (0, 0);
    for i in 0..bvec1.len() {
        let markers1 = read_flag(bvec1[i]);
        let markers2 = read_flag(bvec2[i]);

        for j in 0..8 {
            match (markers1[j], markers2[j]) {
                (true, true) => {
                    distance += (alphas1[spos1] - alphas2[spos2]).abs();
                    spos1 += 1;
                    spos2 += 1;
                },
                (false, true) => {
                    distance += alphas2[spos2];
                    spos2 += 1;
                },
                (true, false) => {
                    distance += alphas1[spos1];
                    spos1 += 1;
                },
                (false, false) => (),
            }// end-match
        } // end-j for
    } // end-i for

    assert_eq!(spos1, alphas1.len());
    assert_eq!(spos2, alphas2.len());

    Ok(distance)
}

fn add_rows(obvec: &Vec<u8>,
            oalphas: &Vec<f32>,
            nbvec: &Vec<u8>,
            nalphas: &Vec<f32>
) -> Result<(Vec<u8>, Vec<f32>), io::Error> {
    assert_eq!(obvec.len(), nbvec.len());
    let (mut spos1, mut spos2) = (0, 0);

    let mut bvec = Vec::new();
    let mut alphas = Vec::new();

    for i in 0..obvec.len() {
        let markers1 = read_flag(obvec[i]);
        let markers2 = read_flag(nbvec[i]);

        bvec.push( obvec[i] | nbvec[i] );
        for j in 0..8 {
            match (markers1[j], markers2[j]) {
                (true, true) => {
                    alphas.push( oalphas[spos1] + nalphas[spos2]);
                    spos1 += 1;
                    spos2 += 1;
                },
                (false, true) => {
                    alphas.push(nalphas[spos2]);
                    spos2 += 1;
                },
                (true, false) => {
                    alphas.push(oalphas[spos1]);
                    spos1 += 1;
                },
                (false, false) => (),
            }// end-match
        } // end-j for
    } // end-i for

    assert_eq!(spos1, oalphas.len());
    assert_eq!(spos2, nalphas.len());

    Ok((bvec, alphas))
}

pub fn generate(bit_vecs: Vec<Vec<u8>>,
                alphas: Vec<Vec<f32>>,
) -> Result<(Vec<Vec<u8>>, Vec<Vec<f32>>), io::Error> {
    info!("generating Priors");
    assert!( bit_vecs.len() == alphas.len() );

    let num_cells = alphas.len();

    // get all-v-all distances
    let mut dists: HashMap<u32, HashMap<u32, f32>> = HashMap::new();
    for i in 0..num_cells {
        let mut cell_dists = HashMap::new();
        for j in 0..num_cells {
            cell_dists.insert( j as u32,
                               get_manhattan_distance(&bit_vecs, &alphas, i, j)?
            );
        }

        dists.insert(i as u32, cell_dists);
    }

    let mut nbit_vecs = Vec::new();
    let mut nalphas = Vec::new();
    for (cell_id, cell_dists) in dists {
        let mut cell_bit_vecs = bit_vecs[cell_id as usize].clone();
        let mut cell_alphas = alphas[cell_id as usize].clone();
        let mut cell_dists_vec: Vec<_> = cell_dists.iter().collect();

        cell_dists_vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Equal));
        for (nidx, (ncell_id, _)) in cell_dists_vec.into_iter().enumerate(){
            if nidx == 0 { continue; }

            let summay_counts = add_rows(&cell_bit_vecs,
                                         &cell_alphas,
                                         &bit_vecs[*ncell_id as usize],
                                         &alphas[*ncell_id as usize])?;
            cell_bit_vecs = summay_counts.0;
            cell_alphas = summay_counts.1;

            if nidx == K_NEAREST { break; } // use only top K_NEAREST
        }

        cell_alphas.iter_mut()
            .for_each(|x| *x /= K_NEAREST as f32);

        nbit_vecs.push(cell_bit_vecs);
        nalphas.push(cell_alphas);
    }

    Ok((nbit_vecs, nalphas))
}
