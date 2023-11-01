use fdg_sim::{
    petgraph::{
        stable_graph::{EdgeIndex, EdgeReference, NodeIndex},
        visit::EdgeRef,
    },
    ForceGraph, ForceGraphHelper,
};
use gpca::third::{self, fdg_macroquad::VisualizationSettings};
use macroquad::prelude::Color;

#[macroquad::main("Lattice Graph")]
async fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();

    graph.add_force_node("origin", ());

    let mut nodes = graph.node_indices().collect();
    let mut edges = graph.edge_indices().collect();

    loop {
        (nodes, edges) = evolve_system(nodes, edges, &mut graph);
        if nodes.len() > 100 {
            break;
        }
    }

    let settings = VisualizationSettings {
        node_color: |_node| Color::from_rgba(255, 255, 255, 255),
        ..Default::default()
    };

    third::fdg_macroquad::run_window(&graph, settings).await;
}

fn evolve_system(
    nodes: Vec<NodeIndex>,
    _edges: Vec<EdgeIndex>,
    graph: &mut ForceGraph<(), ()>,
) -> (Vec<NodeIndex>, Vec<EdgeIndex>) {
    let graph_clone = graph.clone();

    // for each node
    for node in nodes {
        // get neighbors
        let collected_edges: Vec<EdgeReference<'_, ()>> = graph_clone.edges(node).collect();

        println!("{node:?} {:?}", collected_edges.len());

        match collected_edges[..] {
            [] => {
                let north = graph.add_force_node("north", ());
                let east = graph.add_force_node("east", ());
                let south = graph.add_force_node("south", ());
                let west = graph.add_force_node("west", ());

                graph.add_edge(node, north, ());
                graph.add_edge(node, east, ());
                graph.add_edge(node, south, ());
                graph.add_edge(node, west, ());
            }

            // [e1] => {
            //     let n_node = e1.source();

            //     let right = graph.add_force_node("ext_i", ());

            //     graph.add_edge(right, n_node, ());
            // }
            [_e1, _e2] => {
                let up = graph.add_force_node("up", ());
                let down = graph.add_force_node("down", ());

                graph.add_edge(node, up, ());
                graph.add_edge(node, down, ());

                // let ext_1 = graph.add_force_node("ext_a", ());
                // let ext_2 = graph.add_force_node("ext_b", ());

                // let n1 = e1.target();
                // let n2 = e2.target();

                // graph.add_edge(n1, ext_1, ());
                // graph.add_edge(n2, ext_2, ());
            }
            // [e1, e2, e3] => {
            //     let down = graph.add_force_node("down", ());

            //     graph.add_edge(node, down, ());

            //     let corner_a = graph.add_force_node("corner_a", ());
            //     let corner_b = graph.add_force_node("corner_b", ());

            //     let n1 = e1.target();
            //     let n2 = e2.target();
            //     let n3 = e3.target();

            //     graph.add_edge(n1, corner_a, ());
            //     graph.add_edge(n2, corner_a, ());
            //     graph.add_edge(n2, corner_b, ());
            //     graph.add_edge(n3, corner_b, ());
            // }
            [e1, e2, e3, e4] => {
                let n1 = e1.target();
                let n2 = e2.target();
                let n3 = e3.target();
                let n4 = e4.target();

                let n1_edges = graph.edges(n1).count();
                let n2_edges = graph.edges(n2).count();
                let n3_edges = graph.edges(n3).count();
                let n4_edges = graph.edges(n4).count();

                if n1_edges < 2 || n2_edges < 2 {
                    let corner_a = graph.add_force_node("corner_a", ());

                    graph.add_edge(n1, corner_a, ());
                    graph.add_edge(n2, corner_a, ());
                }

                if n2_edges < 2 || n3_edges < 2 {
                    let corner_b = graph.add_force_node("corner_b", ());

                    graph.add_edge(n2, corner_b, ());
                    graph.add_edge(n3, corner_b, ());
                }

                if n3_edges < 2 || n4_edges < 2 {
                    let corner_c = graph.add_force_node("corner_c", ());

                    graph.add_edge(n3, corner_c, ());
                    graph.add_edge(n4, corner_c, ());
                }

                if n4_edges < 2 || n1_edges < 2 {
                    let corner_d = graph.add_force_node("corner_d", ());

                    graph.add_edge(n4, corner_d, ());
                    graph.add_edge(n1, corner_d, ());
                }
            }

            // [e1, e2, e3, e4, e5, ..] => {
            //     let n1 = e1.target();
            //     let n5 = e5.target();

            //     let n1_edges = graph.edges(n1).count();
            //     let n5_edges = graph.edges(n5).count();

            //     if n1_edges == 2 && n5_edges == 2 {
            //         let n4 = e4.target();
            //         graph.remove_node(n5);
            //         graph.remove_edge(e5.id());

            //         graph.add_edge(n1, n4, ());
            //     }
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
