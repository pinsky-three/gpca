use gpca::graphics::{eca_model, update, view};

fn main() {
    nannou::app(eca_model::<48>)
        .update(update)
        .simple_window(view)
        .run();
}
