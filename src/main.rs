use gpca::{
    ds::DynamicalSystemArrayBuilder, dynamics::life::LifeLikeCellularAutomatonArray,
    space::TwoDimensional, utils::save_ds_as_image,
};

fn main() {
    let mut ca = DynamicalSystemArrayBuilder::new(
        TwoDimensional::<512, 512>::new_random(2),
        LifeLikeCellularAutomatonArray::new(&[3], &[2, 3]),
    )
    .build();

    let tps = ca.evolve(60);

    println!("ticks per seconds: {:.2}", tps);

    save_ds_as_image(ca);
}
