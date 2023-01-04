use std::marker::PhantomData;

use convolutions_rs::{
    convolutions::{conv2d, ConvolutionLayer},
    Padding,
};
use ndarray::{Array, Array3, Array4, ArrayBase, Dim, OwnedRepr, RawViewRepr};

use crate::{
    dynamic::{Dynamic, DynamicArray},
    space::{DiscreteSpace, DiscreteSpaceArray, TwoDimensional},
};

#[derive(Debug, Clone)]
pub struct LifeLikeCellularAutomaton<S: DiscreteSpace<2>> {
    b_list: &'static [u32],
    s_list: &'static [u32],
    phantom: PhantomData<S>,
}

#[derive(Debug, Clone)]
pub struct LifeLikeCellularAutomatonArray<S: DiscreteSpaceArray<2>> {
    b_list: &'static [u32],
    s_list: &'static [u32],
    phantom: PhantomData<S>,
}

impl<const X: usize, const Y: usize> LifeLikeCellularAutomaton<TwoDimensional<X, Y>> {
    pub fn new(b_list: &'static [u32], s_list: &'static [u32]) -> Self {
        Self {
            b_list,
            s_list,
            phantom: PhantomData,
        }
    }
}

impl<const X: usize, const Y: usize> LifeLikeCellularAutomatonArray<TwoDimensional<X, Y>> {
    pub fn new(b_list: &'static [u32], s_list: &'static [u32]) -> Self {
        Self {
            b_list,
            s_list,
            phantom: PhantomData,
        }
    }
}

impl<const X: usize, const Y: usize> Dynamic<2, TwoDimensional<X, Y>>
    for LifeLikeCellularAutomaton<TwoDimensional<X, Y>>
{
    fn states(&self) -> u32 {
        2
    }

    fn update(&self, input: &Vec<u32>) -> Vec<u32> {
        let mut output: Vec<u32> = input.to_vec();

        for x in 0..X as i32 {
            for y in 0..Y as i32 {
                let neighbors = {
                    let mut n = 0;

                    for i in [-1, 0, 1] {
                        for j in [-1, 0, 1] {
                            if i == 0 && j == 0 {
                                continue;
                            }

                            let x = (x + i + X as i32) % X as i32;
                            let y = (y + j + Y as i32) % Y as i32;

                            let current_cell = (y * X as i32) + x;

                            n += input[current_cell as usize];
                        }
                    }

                    n
                };

                let current_cell = (y * X as i32) + x;

                if self.b_list.contains(&neighbors) {
                    output[current_cell as usize] = 1;
                } else if self.s_list.contains(&neighbors) {
                    output[current_cell as usize] = input[current_cell as usize];
                } else {
                    output[current_cell as usize] = 0;
                    // println!("set 0");
                }
            }
        }

        output
    }
}

impl<const X: usize, const Y: usize> DynamicArray<2, TwoDimensional<X, Y>>
    for LifeLikeCellularAutomatonArray<TwoDimensional<X, Y>>
{
    fn states(&self) -> u32 {
        2
    }

    fn update(
        &self,
        input: &ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>>,
    ) -> ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>> {
        // Input has shape (channels, height, width)
        // let mut input = ArrayBase::cast::<f32>(input).unwrap();

        println!("starting update");

        let input = input.to_owned().mapv(|x| x as f32);

        println!("input shape: {:?}", input.shape());

        let input = input.into_shape((1, X, Y)).unwrap();

        println!("input shape: {:?}", input.shape());

        let kernel: Array4<f32> =
            Array::from_shape_vec((1, 1, 3, 3), vec![1., 1., 1., 1., 0., 1., 1., 1., 1.]).unwrap();

        let output_free = conv2d(&kernel, None, &input, Padding::Same, 1);

        println!("output shape: {:?}", output_free.shape());

        let output = output_free
            .into_shape((X, Y))
            .unwrap()
            .mapv(|x| x as u32)
            .map(|x| {
                if self.b_list.contains(&x) {
                    1
                } else if self.s_list.contains(&x) {
                    1
                } else {
                    0
                }
            });

        output
    }
}
