use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter, Result},
    hash::Hash,
};

type HyperEdge<E> = (Box<Vec<usize>>, E);

#[derive(Clone)]
pub struct LocalHyperGraph<const D: usize, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Default,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    nodes: Box<[N; D]>,
    edges: HashMap<usize, HyperEdge<E>>,
}

impl<const D: usize, N, E> LocalHyperGraph<D, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Default + Copy,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    pub fn new(nodes: Box<[N; D]>, edges: HashMap<usize, HyperEdge<E>>) -> Self {
        Self { nodes, edges }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Box::new([N::default(); D]),
            edges: HashMap::with_capacity(capacity),
        }
    }
    pub fn is_leaf_node(&self, node: &N) -> bool {
        let count = self
            .edges
            .values()
            .filter(|edge| edge.0.iter().any(|id| &self.nodes[*id] == node))
            .count();
        count == 1
    }

    pub fn add_node(&mut self, node: N) -> usize {
        let index = self.nodes.iter().position(|n| n == &node);
        match index {
            Some(i) => i,
            None => {
                let position = self
                    .nodes
                    .iter()
                    .position(|n| n.clone() == N::default())
                    .unwrap();
                self.nodes[position] = node;
                position
            }
        }
    }

    pub fn find_node_id(&self, node: &N) -> Option<usize> {
        self.nodes.iter().position(|n| n == node)
    }
}

#[derive(Clone)]
pub enum ComplexHyperGraph<const D: usize, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Default,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    SubGraphs(HashMap<HyperEdge<E>, Self>),
    HyperGraph(Box<LocalHyperGraph<D, N, E>>),
}

impl<const D: usize, N, E> ComplexHyperGraph<D, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Default,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    pub fn if_leaf<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&LocalHyperGraph<D, N, E>) -> R,
    {
        match self {
            ComplexHyperGraph::SubGraphs(_) => None,
            ComplexHyperGraph::HyperGraph(hg) => Some(f(hg)),
        }
    }
}

fn create_quadtree<const D: usize>(
    depth: usize,
    current_depth: usize,
) -> ComplexHyperGraph<D, Vec<bool>, ()> {
    if current_depth == depth {
        let nodes_vec: Vec<Vec<bool>> = vec![vec![false; 1 << current_depth]; D];
        let nodes: Box<[Vec<bool>; D]> = match nodes_vec.try_into() {
            Ok(array) => Box::new(array),
            Err(_) => panic!("Failed to convert Vec into Box<[Vec<bool>; D]>"),
        };
        ComplexHyperGraph::HyperGraph(Box::new(LocalHyperGraph {
            nodes,
            edges: HashMap::new(),
        }))
    } else {
        let mut subgraphs = HashMap::new();
        for _ in 0..4 {
            let subgraph = create_quadtree::<D>(depth, current_depth + 1);
            let edge = (Box::new(vec![0]), ());
            subgraphs.insert(edge, subgraph);
        }
        ComplexHyperGraph::SubGraphs(subgraphs)
    }
}

pub fn build_quadtree<const D: usize>(depth: usize) -> ComplexHyperGraph<D, Vec<bool>, ()> {
    create_quadtree::<D>(depth, 0)
}

impl<const D: usize, N, E> Debug for LocalHyperGraph<D, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Debug + Default,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("LocalHyperGraph")
            .field("nodes", &self.nodes)
            .field("edges", &self.edges)
            .finish()
    }
}

impl<const D: usize, N, E> Debug for ComplexHyperGraph<D, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Debug + Default,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ComplexHyperGraph::SubGraphs(subgraphs) => f
                .debug_map()
                .entries(subgraphs.iter().map(|(k, v)| (k, v)))
                .finish(),
            ComplexHyperGraph::HyperGraph(hg) => hg.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Default)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EdgeType {
    ParentChild,
}

fn split_cell<const D: usize>(graph: &mut LocalHyperGraph<D, Cell, EdgeType>, cell: &Cell) {
    let half_width = cell.width / 2;
    let half_height = cell.height / 2;

    let children = [
        Cell {
            x: cell.x,
            y: cell.y,
            width: half_width,
            height: half_height,
        },
        Cell {
            x: cell.x + half_width,
            y: cell.y,
            width: half_width,
            height: half_height,
        },
        Cell {
            x: cell.x,
            y: cell.y + half_height,
            width: half_width,
            height: half_height,
        },
        Cell {
            x: cell.x + half_width,
            y: cell.y + half_height,
            width: half_width,
            height: half_height,
        },
    ];

    let cell_id = graph.nodes.iter().position(|n| n == cell).unwrap();

    for (i, child) in children.iter().enumerate() {
        let child_id = D + i;
        graph.nodes[child_id] = child.clone();

        graph
            .edges
            .insert(child_id, (Box::new(vec![cell_id]), EdgeType::ParentChild));
    }
}

pub fn build_quadtree_2<const D: usize>(depth: usize) -> LocalHyperGraph<D, Cell, EdgeType> {
    let mut graph = LocalHyperGraph {
        nodes: Box::new(
            [Cell {
                x: 0,
                y: 0,
                width: 10,
                height: 10,
            }; D],
        ),
        edges: HashMap::new(),
    };

    for _ in 0..depth {
        let leaf_cells: Vec<Cell> = graph
            .nodes
            .iter()
            .filter(|&node| graph.is_leaf_node(node))
            .cloned()
            .collect();

        for cell in leaf_cells {
            split_cell(&mut graph, &cell);
        }
    }

    graph
}

// fn get_child_cells(cell: &Cell, depth: usize) -> Vec<Cell> {
//     let half_width = cell.width / 2;
//     let half_height = cell.height / 2;

//     vec![
//         Cell {
//             x: cell.x,
//             y: cell.y,
//             width: half_width,
//             height: half_height,
//         },
//         Cell {
//             x: cell.x + half_width,
//             y: cell.y,
//             width: half_width,
//             height: half_height,
//         },
//         Cell {
//             x: cell.x,
//             y: cell.y + half_height,
//             width: half_width,
//             height: half_height,
//         },
//         Cell {
//             x: cell.x + half_width,
//             y: cell.y + half_height,
//             width: half_width,
//             height: half_height,
//         },
//     ]
// }

fn get_child_cells(cell: &Cell) -> Vec<Cell> {
    let half_width = cell.width / 2;
    let half_height = cell.height / 2;

    vec![
        Cell {
            x: cell.x,
            y: cell.y + half_height,
            width: half_width,
            height: half_height,
        },
        Cell {
            x: cell.x + half_width,
            y: cell.y + half_height,
            width: half_width,
            height: half_height,
        },
        Cell {
            x: cell.x + half_width,
            y: cell.y,
            width: half_width,
            height: half_height,
        },
        Cell {
            x: cell.x,
            y: cell.y - half_height,
            width: half_width,
            height: half_height,
        },
    ]
}

// fn build_quadtree_recursive<const D: usize>(
//     graph: &mut LocalHyperGraph<D, Cell, EdgeType>,
//     cell: &Cell,
//     depth: usize,
//     visited: &mut HashSet<Cell>,
// ) {
//     let cell_id = graph.add_node(cell.clone());

//     if depth > 0 {
//         let child_cells = get_child_cells(cell, depth);

//         for child in child_cells {
//             if !visited.contains(&child) {
//                 build_quadtree_recursive(graph, &child, depth - 1, visited);
//                 visited.insert(child.clone()); // Añade el nodo visitado al conjunto visited.
//             }

//             let child_id = graph.find_node_id(&child).unwrap();
//             graph
//                 .edges
//                 .insert(child_id, (Box::new(vec![cell_id]), EdgeType::ParentChild));
//         }
//     }
// }

pub fn build_quadtree_recursive<const D: usize>(
    depth: usize,
    graph: &mut LocalHyperGraph<D, Cell, EdgeType>,
    cell: Cell,
) {
    if depth == 0 {
        return;
    }

    let cell_id = graph.add_node(cell);
    let child_cells = get_child_cells(&cell);

    for child_cell in child_cells {
        let child_id = graph.add_node(child_cell);
        graph
            .edges
            .insert(child_id, (Box::new(vec![cell_id]), EdgeType::ParentChild));
        build_quadtree_recursive(depth - 1, graph, child_cell);
    }
}

// pub fn build_quadtree_3(depth: usize, root_cell: Cell) -> LocalHyperGraph<Cell, EdgeType> {
//     let mut graph = LocalHyperGraph {
//         nodes: vec![root_cell],
//         edges: HashMap::new(),
//     };
//     build_quadtree_recursive(depth, &mut graph, root_cell);
//     graph
// }
