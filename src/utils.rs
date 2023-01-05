use rand::{distributions::Alphanumeric, Rng};

use crate::{
    ds::DynamicalSystemArray, dynamic::DynamicArray, render::ImageSpaceArrayRenderer,
    space::DiscreteSpaceArray,
};

pub fn save_ds_as_image<const D: usize, S, F>(mut ca: DynamicalSystemArray<D, S, F>)
where
    S: DiscreteSpaceArray<D> + ImageSpaceArrayRenderer<D>,
    F: DynamicArray<D, S>,
{
    let img = ca.space().render();

    let entropy: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    let ca_name = format!("renders/{}_{}.png", ca.name(), entropy);

    img.save(&ca_name).unwrap();

    println!("saved image at {}", ca_name);
}
