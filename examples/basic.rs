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

        println!("{node:?} {:?}", collected_edges.len());

        match collected_edges[..] {
            [] => {
                // let right = graph.add_force_node("right", ());
                // let bottom = graph.add_force_node("bottom", ());
                // let diagonal = graph.add_force_node("diagonal", ());

                // graph.add_edge(node, right, ());
                // graph.add_edge(node, bottom, ());

                // graph.add_edge(diagonal, right, ());
                // graph.add_edge(diagonal, bottom, ());

                let n_node = graph.add_force_node("ext", ());

                graph.add_edge(node, n_node, ());
            }

            [e1] => {
                let n_node = e1.source();

                let right = graph.add_force_node("right", ());

                graph.add_edge(right, n_node, ());
            }
            [e1, e2] => {
                // let n_node = e1.target();
                // let s_node = e2.target();

                // let right = graph.add_force_node("right", ());
                // let bottom = graph.add_force_node("bottom", ());
                let diagonal = graph.add_force_node("diagonal", ());

                graph.add_edge(node, diagonal, ());
                // graph.add_edge(node, bottom, ());

                // graph.add_edge(diagonal, right, ());
                // graph.add_edge(diagonal, bottom, ());

                // graph.add_edge(n_node, right, ());
                // graph.add_edge(s_node, bottom, ());

                // graph.add_edge(node, diagonal, ());
            }

            [e1, e2, e3] => {
                // let n1_node = e1.target();
                // let n2_node = e2.target();
                // let n3_node = e3.target();

                let diagonal = graph.add_force_node("diagonal", ());

                // graph.add_edge(node, diagonal, ());
                // graph.add_edge(diagonal, n1_node, ());
                // graph.add_edge(diagonal, n2_node, ());
                // graph.add_edge(n2_node, node, ());

                // let right = graph.add_force_node("right", ());
                // let bottom = graph.add_force_node("bottom", ());
                // let diagonal = graph.add_force_node("diagonal", ());

                // graph.add_edge(node, right, ());
                // graph.add_edge(node, bottom, ());

                graph.add_edge(node, diagonal, ());
                // graph.add_edge(diagonal, bottom, ());

                // graph.add_edge(n_node, right, ());
                // graph.add_edge(s_node, bottom, ());
                // graph.add_edge(e_node, diagonal, ());
            }
            // [e1, e2, e3, e4] => {
            //     let n_node = e1.target();
            //     let e_node = e2.target();
            //     let s_node = e3.target();
            //     let w_node = e4.target();

            //     // let max_neighbors = 4;

            //     // if graph.edges(n_node).count() < max_neighbors {
            //     let n1 = graph.add_force_node("nw", ());

            //     graph.add_edge(n_node, n1, ());
            //     graph.add_edge(n1, w_node, ());
            //     // };

            //     // if graph.edges(s_node).count() < max_neighbors {
            //     let n2 = graph.add_force_node("ne", ());

            //     graph.add_edge(n_node, n2, ());
            //     graph.add_edge(n2, e_node, ());
            //     // }

            //     // if graph.edges(e_node).count() < max_neighbors {
            //     let n3 = graph.add_force_node("se", ());

            //     graph.add_edge(s_node, n3, ());
            //     graph.add_edge(n3, e_node, ());
            //     // }

            //     // if graph.edges(w_node).count() < max_neighbors {
            //     let n4 = graph.add_force_node("sw", ());

            //     graph.add_edge(s_node, n4, ());
            //     graph.add_edge(n4, w_node, ());
            //     // }

            //     // let diagonal = graph.add_force_node("diagonal", ());

            //     // graph.add_edge(n_node, diagonal, ());
            //     // graph.add_edge(s_node, diagonal, ());
            //     // graph.add_edge(e_node, s_node, ());
            // }
            // [e1] => {
            //     graph.remove_node(node);
            //     graph.remove_edge(e1.id());
            // }
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
