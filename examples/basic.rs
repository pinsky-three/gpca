// use image::{ImageBuffer, Rgb, RgbImage};

// use kdam::tqdm;
// use rand::{thread_rng, Rng};
// use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use fdg_sim::{
    petgraph::{
        stable_graph::{EdgeIndex, EdgeReference, NodeIndex},
        visit::{EdgeRef, IntoEdges, NodeCount},
    },
    ForceGraph, ForceGraphHelper,
};
use gpca::third;

#[macroquad::main("Lattice Graph")]
async fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();
    // let mut indices: Vec<NodeIndex> = Vec::new();

    // let n_s = next_state(0, vec![]);
    // println!("{:?}", n_s);

    graph.add_force_node("origin", ());

    let mut nodes = graph.node_indices().collect();
    let mut edges = graph.edge_indices().collect();

    // (nodes, edges) = evolve_system(nodes, edges, &mut graph);
    // (nodes, edges) = evolve_system(nodes, edges, &mut graph);
    // (nodes, edges) = evolve_system(nodes, edges, &mut graph);
    // (nodes, edges) = evolve_system(nodes, edges, &mut graph);

    loop {
        (nodes, edges) = evolve_system(nodes, edges, &mut graph);
        if nodes.len() > 1000 {
            break;
        }
    }

    third::fdg_macroquad::run_window(&graph).await;
}

fn evolve_system(
    nodes: Vec<NodeIndex>,
    _edges: Vec<EdgeIndex>,
    graph: &mut ForceGraph<(), ()>,
) -> (Vec<NodeIndex>, Vec<EdgeIndex>) {
    for node in nodes {
        // for each node
        let graph_clone = graph.clone();
        let edges = graph_clone.edges(node);

        // let total_edges = edges.clone().count();
        let collected_edges = edges.collect::<Vec<EdgeReference<'_, ()>>>();

        // println!("{node:?} {:?}", collected_edges.len());

        match collected_edges[..] {
            [] => {
                let right = graph.add_force_node("right", ());
                let bottom = graph.add_force_node("bottom", ());
                let diagonal = graph.add_force_node("diagonal", ());

                graph.add_edge(node, right, ());
                graph.add_edge(node, bottom, ());

                graph.add_edge(diagonal, right, ());
                graph.add_edge(diagonal, bottom, ());
            }

            [e1, e2] => {
                let n_node = e1.target();
                let s_node = e2.target();

                let right = graph.add_force_node("right", ());
                let bottom = graph.add_force_node("bottom", ());
                let diagonal = graph.add_force_node("diagonal", ());

                graph.add_edge(node, right, ());
                graph.add_edge(node, bottom, ());

                graph.add_edge(diagonal, right, ());
                graph.add_edge(diagonal, bottom, ());

                graph.add_edge(n_node, right, ());
                graph.add_edge(s_node, bottom, ());
            }

            _ => {
                // println!("nothing");
            }
        }
    }

    (
        graph.node_indices().collect::<Vec<NodeIndex>>(),
        graph.edge_indices().collect::<Vec<EdgeIndex>>(),
    )
}
