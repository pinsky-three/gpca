use egui_macroquad::macroquad::color::Color;
use fdg_sim::{
    petgraph::{
        stable_graph::{EdgeReference, NodeIndex},
        visit::EdgeRef,
    },
    ForceGraph, ForceGraphHelper,
};
use gpca::third::{self, fdg_macroquad::VisualizationSettings};
// use macroquad::prelude::Color;
// use tokio::signal::unix::Signal;

#[macroquad::main("Lattice Graph")]
async fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();

    graph.add_force_node("origin", ());

    loop {
        evolve_system(&mut graph);
        // evolve_system(&mut graph);
        // evolve_system(&mut graph);
        // evolve_system(&mut graph);
        // evolve_system(&mut graph);
        if graph.node_indices().count() > 5000 {
            break;
        }
    }

    let settings = VisualizationSettings {
        node_color: |_node| Color::from_rgba(255, 255, 255, 255),
        ..Default::default()
    };

    third::fdg_macroquad::run_window(&graph, settings).await;
}

pub trait Interaction {
    fn interact<E>(&self, node: NodeIndex, edges: Vec<EdgeReference<'_, E>>) -> Vec<Signal<(), E>>;
}

pub enum Signal<N, E> {
    CreateNode(&'static str, N),
    CreateEdge {
        a: NodeIndex,
        b: NodeIndex,
        weight: E,
    },
}

// type GraphByEdge = Vec<Vec<u32>>;

// fn transform(graph: GraphByEdge) -> GraphByEdge {
//     match graph.as_slice() {
//         [abc, def, [foo]] => {}
//         _ => {}
//     }

//     todo!()
// }

pub struct PseudoLatticeGraph;

impl Interaction for PseudoLatticeGraph {
    fn interact<E>(
        &self,
        _node: NodeIndex,
        edges: Vec<EdgeReference<'_, E>>,
    ) -> Vec<Signal<(), E>> {
        // let collected_edges: Vec<EdgeReference<'_, ()>> = edges.collect();
        let signals = Vec::new();

        match edges[..] {
            [] => {
                // let north = signals.push(Signal::CreateNode("north", ()));
                // let east = graph.add_force_node("east", ());
                // let south = graph.add_force_node("south", ());
                // let west = graph.add_force_node("west", ());

                // graph.add_edge(node, north, ());
                // graph.add_edge(node, east, ());
                // graph.add_edge(node, south, ());
                // graph.add_edge(node, west, ());
            }

            [_e1, _e2] => {
                // let up = graph.add_force_node("up", ());
                // let down = graph.add_force_node("down", ());

                // graph.add_edge(node, up, ());
                // graph.add_edge(node, down, ());
            }

            // [e1, e2, e3, e4] => {
            // let n1 = e1.target();
            // let n2 = e2.target();
            // let n3 = e3.target();
            // let n4 = e4.target();

            // if graph.edges(n1).count() < 2 || graph.edges(n2).count() < 2 {
            //     let corner_a = graph.add_force_node("corner_a", ());

            //     graph.add_edge(n1, corner_a, ());
            //     graph.add_edge(n2, corner_a, ());
            // }

            // if graph.edges(n2).count() < 2 || graph.edges(n3).count() < 2 {
            //     let corner_b = graph.add_force_node("corner_b", ());

            //     graph.add_edge(n2, corner_b, ());
            //     graph.add_edge(n3, corner_b, ());
            // }

            // if graph.edges(n3).count() < 2 || graph.edges(n4).count() < 2 {
            //     let corner_c = graph.add_force_node("corner_c", ());

            //     graph.add_edge(n3, corner_c, ());
            //     graph.add_edge(n4, corner_c, ());
            // }

            // if graph.edges(n4).count() < 2 || graph.edges(n1).count() < 2 {
            //     let corner_d = graph.add_force_node("corner_d", ());

            //     graph.add_edge(n4, corner_d, ());
            //     graph.add_edge(n1, corner_d, ());
            // }
            // }
            _ => {}
        }

        signals
    }
}

fn evolve_system(graph: &mut ForceGraph<(), ()>) {
    let graph_clone = graph.clone();

    // for each node
    for node in graph_clone.node_indices() {
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

            [_e1, _e2] => {
                let up = graph.add_force_node("up", ());
                let down = graph.add_force_node("down", ());

                graph.add_edge(node, up, ());
                graph.add_edge(node, down, ());
            }

            [e1, e2, e3, e4] => {
                let n1 = e1.target();
                let n2 = e2.target();
                let n3 = e3.target();
                let n4 = e4.target();

                if graph.edges(n1).count() < 2 || graph.edges(n2).count() < 2 {
                    let corner_a = graph.add_force_node("corner_a", ());

                    graph.add_edge(n1, corner_a, ());
                    graph.add_edge(n2, corner_a, ());
                }

                if graph.edges(n2).count() < 2 || graph.edges(n3).count() < 2 {
                    let corner_b = graph.add_force_node("corner_b", ());

                    graph.add_edge(n2, corner_b, ());
                    graph.add_edge(n3, corner_b, ());
                }

                if graph.edges(n3).count() < 2 || graph.edges(n4).count() < 2 {
                    let corner_c = graph.add_force_node("corner_c", ());

                    graph.add_edge(n3, corner_c, ());
                    graph.add_edge(n4, corner_c, ());
                }

                if graph.edges(n4).count() < 2 || graph.edges(n1).count() < 2 {
                    let corner_d = graph.add_force_node("corner_d", ());

                    graph.add_edge(n4, corner_d, ());
                    graph.add_edge(n1, corner_d, ());
                }
            }

            // [e1, e2, e3, e4, e5, ..] => {
            //     let n1 = e2.target();
            //     let n5 = e4.target();

            //     println!("n1: {}", n1.index());
            //     println!("n5: {}", n5.index());

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
}
