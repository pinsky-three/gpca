use gpca::{
    ds::DynamicalSystemArrayBuilder, dynamics::life::LifeLikeCellularAutomatonArray,
    space::TwoDimensional, utils::save_ds_as_image,
};

fn main() {
    let space = TwoDimensional::<2048, 2048>::new_random(2);

    let dynamic = LifeLikeCellularAutomatonArray::new(&[3], &[2, 3]);

    let mut ca = DynamicalSystemArrayBuilder::new(space, dynamic).build();

    let tps = ca.evolve(320);

    println!("ticks per seconds: {:.2}", tps);

    save_ds_as_image(ca);
}
