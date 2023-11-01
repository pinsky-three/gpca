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
        if nodes.len() > 1000 {
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
                let n_node = graph.add_force_node("ext_0", ());

                graph.add_edge(node, n_node, ());
            }

            [e1] => {
                let n_node = e1.source();

                let right = graph.add_force_node("ext_i", ());

                graph.add_edge(right, n_node, ());
            }
            [e1, e2] => {
                let up = graph.add_force_node("up", ());

                graph.add_edge(node, up, ());

                let ext_1 = graph.add_force_node("ext_a", ());
                let ext_2 = graph.add_force_node("ext_b", ());

                let n1 = e1.target();
                let n2 = e2.target();

                graph.add_edge(n1, ext_1, ());
                graph.add_edge(n2, ext_2, ());
            }
            [e1, e2, e3] => {
                let down = graph.add_force_node("down", ());

                graph.add_edge(node, down, ());

                let corner_a = graph.add_force_node("corner_a", ());
                let corner_b = graph.add_force_node("corner_b", ());

                let n1 = e1.target();
                let n2 = e2.target();
                let n3 = e3.target();

                graph.add_edge(n1, corner_a, ());
                graph.add_edge(n2, corner_a, ());
                graph.add_edge(n2, corner_b, ());
                graph.add_edge(n3, corner_b, ());
            }
            [e1, e2, e3, e4] => {
                // let left = graph.add_force_node("left", ());
                // graph.add_edge(node, left, ());

                let corner_a = graph.add_force_node("corner_a", ());
                let corner_b = graph.add_force_node("corner_b", ());
                let corner_c = graph.add_force_node("corner_c", ());
                let corner_d = graph.add_force_node("corner_d", ());

                let n1 = e1.target();
                let n2 = e2.target();
                let n3 = e3.target();
                let n4 = e4.target();

                graph.add_edge(n1, corner_a, ());
                graph.add_edge(n2, corner_a, ());
                graph.add_edge(n2, corner_b, ());
                graph.add_edge(n3, corner_b, ());
                graph.add_edge(n3, corner_c, ());
                graph.add_edge(n4, corner_c, ());
                graph.add_edge(n4, corner_d, ());
                graph.add_edge(n1, corner_d, ());
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
