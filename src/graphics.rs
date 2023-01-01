use nannou::prelude::*;
pub struct Model {}

pub fn model(_app: &App) -> Model {
    Model {}
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {}

pub fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    // Generate sine wave data based on the time of the app
    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();

    // Get boundary of the window (to constrain the movements of our circle)
    let boundary = app.window_rect();

    // Map the sine wave functions to ranges between the boundaries of the window
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    draw.background().color(BLACK);
    draw.rect()
        .x_y(x, y)
        .w_h(20.0, 20.0)
        .color(WHITE)
        .stroke(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
