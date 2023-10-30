// use image::{ImageBuffer, Rgb, RgbImage};

// use kdam::tqdm;
// use rand::{thread_rng, Rng};
// use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use fdg_sim::{
    petgraph::{
        stable_graph::{EdgeIndex, EdgeReference, NodeIndex},
        visit::EdgeRef,
    },
    ForceGraph, ForceGraphHelper,
};

#[macroquad::main("Lattice Graph")]
async fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();
    let mut indices: Vec<NodeIndex> = Vec::new();

    let n_s = next_state(0, vec![]);
    println!("{:?}", n_s);

    let current_node = graph.add_force_node(format!("1"), ());

    for (node_identifier, edges) in n_s {
        match node_identifier {
            CURRENT_NODE => println!("CURRENT_NODE"),
            NEW_NODE => {
                let new_node = graph.add_force_node("name", ());

                for edge in edges {
                    // if edge
                    // graph.add_edge(a, b, weight)
                }

                // graph.add_edge(current_node, new_node)
            }
            _ => println!("OTHER"),
        }
    }
}

// const O: usize = 0;

const CURRENT_NODE: usize = 0;
const NEW_NODE: usize = 1;

fn next_state(_node: usize, edges: Vec<usize>) -> Vec<(usize, Vec<usize>)> {
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

fn evolve_system(
    nodes: Vec<NodeIndex>,
    edges: Vec<EdgeIndex>,
    graph: &mut ForceGraph<(), ()>,
) -> (Vec<NodeIndex>, Vec<EdgeIndex>) {
    for node in nodes {
        // for each node
        let graph_clone = graph.clone();
        let edges = graph_clone.edges(node);

        // let total_edges = edges.clone().count();

        match edges.collect::<Vec<EdgeReference<'_, ()>>>()[..] {
            [] => {
                let n1 = graph.add_force_node("n", ());
                let n2 = graph.add_force_node("s", ());
                let n3 = graph.add_force_node("e", ());
                let n4 = graph.add_force_node("w", ());

                graph.add_edge(node, n1, ());
                graph.add_edge(node, n2, ());
                graph.add_edge(node, n3, ());
                graph.add_edge(node, n4, ());
            }
            [e1, e2, e3, e4] => {
                let n1 = graph.add_force_node("nw", ());
                let n2 = graph.add_force_node("ne", ());
                let n3 = graph.add_force_node("se", ());
                let n4 = graph.add_force_node("sw", ());

                let n_node = e1.target();
                let s_node = e2.target();
                let e_node = e3.target();
                let w_node = e4.target();

                graph.add_edge(n_node, n1, ());
                graph.add_edge(n1, w_node, ());

                graph.add_edge(n_node, n2, ());
                graph.add_edge(n2, e_node, ());

                graph.add_edge(s_node, n3, ());
                graph.add_edge(n3, e_node, ());

                graph.add_edge(s_node, n4, ());
                graph.add_edge(n4, w_node, ());
            }
            [e1, e2, e3] => {
                let n1 = graph.add_force_node("extension", ());

                let current_node = e1.source();

                graph.add_edge(current_node, n1, ());
            }
            _ => {
                println!("nothing");
            }
        }
    }

    (
        graph.node_indices().collect::<Vec<NodeIndex>>(),
        graph.edge_indices().collect::<Vec<EdgeIndex>>(),
    )
}
