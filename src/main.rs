use ds::DynamicalSystemBuilder;
// use gpca::{model, update, view};

mod ds;

fn main() {
    let mut ca = DynamicalSystemBuilder::new(
        ds::OneDimensional(10),
        ds::ElementaryCellularAutomaton::new_from_number(30),
    )
    .with_initial_state(vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0])
    .build();

    for _ in 0..10 {
        println!("{:?}", ca.space());
        ca.tick();
    }

    // println!("{:?}", ca.space());

    // nannou::app(model).update(update).simple_window(view).run();
}
