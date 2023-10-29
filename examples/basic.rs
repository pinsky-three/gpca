// use image::{ImageBuffer, Rgb, RgbImage};

// use kdam::tqdm;
// use rand::{thread_rng, Rng};
// use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use rayon::vec;

#[tokio::main]
async fn main() {
    next_state(0, vec![]);

    todo!()
}

const O: usize = 0;

const CURRENT_NODE: usize = 1;
const NEW_NODE: usize = 2;

fn next_state(node: usize, edges: Vec<usize>) -> Vec<(usize, Vec<usize>)> {
    // match (node, edges[..]) {

    match edges[..] {
        [] => vec![
            (NEW_NODE, vec![CURRENT_NODE, 1]),
            (NEW_NODE, vec![CURRENT_NODE, 2]),
            (NEW_NODE, vec![CURRENT_NODE, 3]),
            (NEW_NODE, vec![CURRENT_NODE, 4]),
        ],
        [1, 2, 3, 4] => vec![
            (NEW_NODE, vec![1, 2]),
            (NEW_NODE, vec![2, 3]),
            (NEW_NODE, vec![1, 4]),
            (NEW_NODE, vec![3, 4]),
        ],
        [1, 2, 3] => vec![(NEW_NODE, vec![CURRENT_NODE, 4])],
        _ => vec![(0, vec![])],
    }
}
