use gpca::{
    ds::DynamicalSystemBuilder,
    dynamic::ElementaryCellularAutomaton,
    space::{DiscreteSpace, OneDimensional},
};

use nannou::rand::random;

fn main() {
    let mut mem = [0; 14];

    for i in 0..mem.len() {
        mem[i] = if random::<bool>() { 1 } else { 0 };
    }

    let space = OneDimensional::new_with_state(mem);
    let dynamic = ElementaryCellularAutomaton::new_from_number(30);

    let mut ca = DynamicalSystemBuilder::new(space, dynamic).build();

    for _ in 0..10 {
        println!("{:?}", ca.space().read_state());
        ca.tick();
    }
}
