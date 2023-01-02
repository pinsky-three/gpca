use itertools::Itertools;
use nannou::prelude::*;

use crate::{
    ds::{DynamicalSystem, DynamicalSystemBuilder},
    dynamic::ElementaryCellularAutomaton,
    space::{DiscreteSpace, OneDimensional},
};
pub struct Model<const X: usize> {
    ca: DynamicalSystem<1, OneDimensional<X>, ElementaryCellularAutomaton<1, OneDimensional<X>>>,

    ca_evolutions: Vec<Vec<u32>>,
}

pub fn eca_model<const X: usize>(app: &App) -> Model<X> {
    let mut mem: [u32; X] = [0; X];

    let init = "bregy"
        .as_bytes()
        .iter()
        .map(|l| format!("{:08b}", l).chars().collect::<Vec<char>>())
        .flatten()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    mem[..init.len()].copy_from_slice(&init);

    let space = OneDimensional::new_with_state(mem);
    // space.write_state(&input);

    let dynamic = ElementaryCellularAutomaton::new_from_number(30);

    let mut ca = DynamicalSystemBuilder::new(space, dynamic).build();

    let mut ca_evolutions = vec![];

    for _ in 0..150 {
        ca_evolutions.push(ca.space().read_state());
        ca.tick();
    }

    let rows: Vec<String> = ca_evolutions
        .iter()
        .map(|v| {
            v.chunks(8).map(|b| {
                let bytes = b.iter().map(|i| i.to_u8().unwrap()).collect::<Vec<u8>>();
                String::from_utf8_lossy(bytes.as_slice()).to_string()
            })
        })
        .flatten()
        .collect();
    // .counts();

    // println!("{:?}", rows);
    // .collect::<Vec<String>>();O
    // rows.

    Model { ca, ca_evolutions }
}

pub fn update<const X: usize>(app: &App, model: &mut Model<X>, update: Update) {
    // let a = app.time.sin().round();
    // if a == 1.0 {
    //     println!("tick");
    //     model.ca.tick();
    // }
}

pub fn view<const X: usize>(app: &App, model: &Model<X>, frame: Frame) {
    let draw = app.draw();

    // // Generate sine wave data based on the time of the app
    // let sine = app.time.sin();
    // let slowersine = (app.time / 2.0).sin();

    // // Get boundary of the window (to constrain the movements of our circle)
    // let boundary = app.window_rect();

    // // Map the sine wave functions to ranges between the boundaries of the window
    // let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    // let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    // draw.background().color(BLACK);
    // draw.rect()
    //     .x_y(x, y)
    //     .w_h(20.0, 20.0)
    //     .color(WHITE)
    //     .stroke(BLACK);

    let win = app.window_rect();

    const W: f32 = 10.0;
    const H: f32 = W;

    let r = Rect::from_w_h(W, H).top_left_of(win);
    // .mid_top_of(win)
    // .middle_of(win)
    // .pad_left(W / 2.0)
    // .pad_top(H / 2.0);

    // .pad_top((app.time * 30.0).floor());

    model.ca_evolutions.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &cell)| {
            if cell == 1 {
                draw.rect()
                    .x_y(r.x() + (j as f32 * W), r.y() - (i as f32 * H))
                    .w_h(W, H)
                    .color(match j {
                        0..=8 => BLUE,
                        9..=16 => RED,
                        17..=24 => GREEN,
                        25..=32 => YELLOW,
                        _ => WHITE,
                    });
            }
        });
    });

    draw.to_frame(app, &frame).unwrap();
}
