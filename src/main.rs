use ds::DynamicalSystemBuilder;
use gpca::{model, update, view};

mod ds;

fn main() {
    let ca = DynamicalSystemBuilder::new(
        ds::OneDimensional(100),
        ds::ElementaryCellularAutomaton::new([0, 1, 0, 1, 0, 1, 1, 1]),
    )
    .build();

    println!("{:?}", ca);

    nannou::app(model).update(update).simple_window(view).run();
}
