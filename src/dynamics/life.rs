use crate::{
    dynamic::{Dynamic, DynamicArray},
    space::{DiscreteSpace, DiscreteSpaceArray, TwoDimensional},
};
use ndarray::{Array, Array2, ArrayBase, Dim, OwnedRepr};
use ndarray_conv::Conv2DExt;
use std::marker::PhantomData;

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
    fn name(&self) -> String {
        format!(
            "lifelike_b{}_s{}",
            self.b_list
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(""),
            self.s_list
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }

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
    fn name(&self) -> String {
        format!(
            "lifelike_b{}_s{}",
            self.b_list
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(""),
            self.s_list
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }

    fn states(&self) -> u32 {
        2
    }

    fn update(
        &self,
        input: &ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>>,
    ) -> ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>> {
        let kernel: Array2<u32> =
            Array::from_shape_vec((3, 3), vec![1, 1, 1, 1, 0, 1, 1, 1, 1]).unwrap();

        let neighbors = input.conv_2d(&kernel).unwrap();

        let survivors = neighbors.mapv(|x| if self.s_list.contains(&x) { 1 } else { 0 }) & input;
        let born = neighbors.mapv(|x| if self.b_list.contains(&x) { 1 } else { 0 });

        survivors | born
    }
}
