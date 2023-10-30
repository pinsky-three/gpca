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
                let n = graph.add_force_node("n", ());
                let s = graph.add_force_node("s", ());
                let e = graph.add_force_node("e", ());
                let w = graph.add_force_node("w", ());

                graph.add_edge(node, n, ());
                graph.add_edge(node, s, ());
                graph.add_edge(node, e, ());
                graph.add_edge(node, w, ());
            }

            [e1, e2, e3, e4] => {
                // println!(".., 4");

                let n_node = e1.target();
                let s_node = e2.target();
                let e_node = e3.target();
                let w_node = e4.target();

                // if graph.edges(n_node).count() > 3
                //     || graph.edges(s_node).count() > 3
                //     || graph.edges(e_node).count() > 3
                //     || graph.edges(w_node).count() > 3
                // {
                //     continue;
                // }

                let max_neighbors = 3;

                if graph.edges(n_node).count() < max_neighbors {
                    let n1 = graph.add_force_node("nw", ());

                    graph.add_edge(n_node, n1, ());
                    graph.add_edge(n1, w_node, ());
                };

                if graph.edges(s_node).count() < max_neighbors {
                    let n2 = graph.add_force_node("ne", ());

                    graph.add_edge(n_node, n2, ());
                    graph.add_edge(n2, e_node, ());
                }

                if graph.edges(e_node).count() < max_neighbors {
                    let n3 = graph.add_force_node("se", ());

                    graph.add_edge(s_node, n3, ());
                    graph.add_edge(n3, e_node, ());
                }

                if graph.edges(w_node).count() < max_neighbors {
                    let n4 = graph.add_force_node("sw", ());

                    graph.add_edge(s_node, n4, ());
                    graph.add_edge(n4, w_node, ());
                }
            }

            [e1, e2, e3] => {
                let current_node = e1.source();

                // let l1 = graph.edges(e1.target()).count();
                // let l2 = graph.edges(e2.target()).count();
                // let l3 = graph.edges(e3.target()).count();

                // if l1 > 5 || l2 > 5 || l3 > 5 {
                //     continue;
                // }

                let n1 = graph.add_force_node("extension", ());

                graph.add_edge(current_node, n1, ());
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
