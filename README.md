
# GPCA (General-Purpose Cellular Automata)

**General-Purpose Cellular Automata (GPCA)** is a Rust implementation of the computational model known as **Async Hyper-Graph Cellular Automata**. This library provides a framework for simulating and experimenting with complex systems through autómata cellular dynamics, utilizing parallel computing and GPU acceleration for enhanced performance.

## Features

- **Async Hyper-Graph Cellular Automata**: Advanced cellular automata model that operates on hypergraphs asynchronously.
- **Cyclic Cellular Automata**: Simulation of cellular automata with cyclic states and customizable thresholds.
- **Life-like Cellular Automata**: Variants of Conway's Game of Life, with fully customizable birth and survival rules.
- **Elementary Cellular Automata**: Implementation of elementary 1D cellular automata with binary rules.
- **GPU Acceleration**: Utilizes `wgpu` for GPU-accelerated computations and simulations.
- **Parallel Processing**: Leveraging `rayon` for parallel computation, ensuring efficient performance on multi-core systems.
- **Visualization**: Easily create images of simulation states, with customizable color gradients and mapping.
- **2D and 3D support** (upcoming): Current support for 2D automata with a planned extension to 3D models.

## Example

```shell
cargo run --example latest --features=rand
```

## Installation

To use **GPCA** in your Rust project, add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
gpca = { version = "0.1.0"}
```

## Example

Below is an example that demonstrates how to simulate a 2D cyclic cellular automaton with 8 states:

```rust
use gpca::{
    dynamics::implementations::cyclic::CyclicAutomaton,
    spaces::implementations::basic::{DiscreteState, HyperGraphHeap},
    system::{dynamical_system::DynamicalSystem, utils::save_space_as_image},
    third::wgpu::create_gpu_device,
};

use kdam::tqdm;

#[tokio::main]
async fn main() {
    const W: u32 = 512;
    const H: u32 = 512;

    const STATES: u32 = 4;
    const THRESH: u32 = 2;

    let _device = create_gpu_device();

    let mem = DiscreteState::filled_vector(W * H, STATES);
    let space = HyperGraphHeap::new_grid(&mem, W, H, ());

    let dynamic = CyclicAutomaton::new(STATES, THRESH);

    let mut system = DynamicalSystem::new(Box::new(space), Box::new(dynamic));

    for _ in tqdm!(0..500) {
        system.compute_sync_wgpu(&_device);
    }

    save_space_as_image(&system, colorous::PLASMA);
}

```

## Project Structure

- **dynamics/**: Contains the implementations of various cellular automata models.
  - **cyclic.rs**: Cyclic automaton implementation.
  - **life.rs**: Life-like automaton implementation.
  - **eca.rs**: Elementary cellular automata.
- **spaces/**: Defines the hypergraph space and the lattice structure for automata to operate in.
- **system/**: The dynamical system that governs the updates and evolution of the automata.
- **third/**: Contains GPU-related functionality, including shaders for computation.

## Future Plans

- **3D Cellular Automata**: Extend support for 3D hyper-graph cellular automata.
- **Advanced Visualization**: Introduce real-time interactive visualizations for cellular automata using WebGPU.
- **Rule-based Cellular Automata**: Support for custom rule definitions via user input.

## Contributions

Contributions are welcome! Feel free to open an issue or submit a pull request with new features, bug fixes, or improvements.

## License

This project is licensed under the MIT License.
