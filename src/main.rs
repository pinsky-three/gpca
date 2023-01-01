use ds::DynamicalSystemBuilder;
use dynamic::ElementaryCellularAutomaton;
use space::OneDimensional;

use crate::space::DiscreteSpace;
// use gpca::{model, update, view};

mod ds;
mod dynamic;
mod space;

fn main() {
    let space = OneDimensional::new_with_state([0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1]);
    let dynamic = ElementaryCellularAutomaton::new_from_number(30);

    let mut ca = DynamicalSystemBuilder::new(space, dynamic).build();

    for _ in 0..10 {
        println!("{:?}", ca.space().read_state());
        ca.tick();
    }

    // nannou::app(model).update(update).simple_window(view).run();
}
