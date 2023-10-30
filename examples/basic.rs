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
use gpca::third;

#[macroquad::main("Lattice Graph")]
async fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();
    // let mut indices: Vec<NodeIndex> = Vec::new();

    // let n_s = next_state(0, vec![]);
    // println!("{:?}", n_s);

    graph.add_force_node("origin", ());

    let nodes = graph.node_indices().collect();
    let edges = graph.edge_indices().collect();

    let (nodes, edges) = evolve_system(nodes, edges, &mut graph);
    let (nodes, edges) = evolve_system(nodes, edges, &mut graph);
    // let (nodes, edges) = evolve_system(nodes, edges, &mut graph);
    // let (nodes, edges) = evolve_system(nodes, edges, &mut graph);
    // let (nodes, edges) = evolve_system(nodes, edges, &mut graph);

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

        match collected_edges[..] {
            [] => {
                let n = graph.add_force_node("n", ());
                let s = graph.add_force_node("s", ());
                let e = graph.add_force_node("e", ());
                let w = graph.add_force_node("w", ());

                graph.add_edge(node, n, ());
                graph.add_edge(node, s, ());
                graph.add_edge(node, e, ());
                graph.add_edge(node, w, ());
            }
            [e1, e2] => {
                println!("corner");
                let n1 = graph.add_force_node("corner", ());

                let l1 = e1.target();
                let l2 = e2.target();

                graph.add_edge(l1, n1, ());
                graph.add_edge(n1, l2, ());
            }
            [e1, e2, e3] => {
                let n1 = graph.add_force_node("extension", ());

                let current_node = e1.source();

                graph.add_edge(current_node, n1, ());
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
