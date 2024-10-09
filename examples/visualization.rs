use fdg_sim::{petgraph::graph::NodeIndex, ForceGraph, ForceGraphHelper};
use gpca::third::{self, fdg_macroquad::VisualizationSettings};

#[egui_macroquad::macroquad::main("Lattice Graph")]
async fn main() {
    let mut graph: ForceGraph<(), ()> = ForceGraph::default();
    let mut indices: Vec<NodeIndex> = Vec::new();

    let size = 20;

    for x in 0..size {
        for y in 0..size {
            let node = graph.add_force_node(format!("{x},{y}"), ());

            indices.push(node);
        }
    }

    for y in 0..size {
        for x in 0..size {
            if x != 0 {
                graph.add_edge(indices[(size * y) + x], indices[((size * y) + x) - 1], ());
            }

            if y != 0 {
                graph.add_edge(indices[(size * y) + x], indices[(size * (y - 1)) + x], ());
            }
        }
    }

    third::fdg_macroquad::run_window(&graph, VisualizationSettings::default()).await;
}
