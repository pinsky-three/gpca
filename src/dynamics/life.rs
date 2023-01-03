use std::marker::PhantomData;

use crate::{
    dynamic::Dynamic,
    space::{DiscreteSpace, TwoDimensional},
};

#[derive(Debug, Clone)]
pub struct LifeLikeCellularAutomaton<S: DiscreteSpace<2>> {
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

impl<const X: usize, const Y: usize> Dynamic<2, TwoDimensional<X, Y>>
    for LifeLikeCellularAutomaton<TwoDimensional<X, Y>>
{
    fn states(&self) -> u32 {
        2
    }

    fn update(&self, input: &Vec<u32>) -> Vec<u32> {
        let mut output: Vec<u32> = (*input).to_vec();

        for x in 0..X {
            for y in 0..Y {
                let mut neighbors = 0;
                let current_cell = (y * X) + x;

                for i in [-1, 0, 1] {
                    for j in [-1, 0, 1] {
                        if i == 0 && j == 0 {
                            continue;
                        }

                        let x = match x as i32 + i {
                            x if x < 0 => X as i32 - 1,
                            x if x >= X as i32 => 0,
                            _ => x as i32,
                        };

                        let y = match y as i32 + j {
                            y if y < 0 => Y as i32 - 1,
                            y if y >= Y as i32 => 0,
                            _ => y as i32,
                        };

                        // let x = (x as i32 + i) as usize;
                        // let y = (y as i32 + j) as usize;

                        // let x = if x >= X { x - X } else { x };
                        // let y = if y >= Y { y - Y } else { y };

                        let index = (y as usize * X) + x as usize;

                        // println!("{index}");
                        // println!("{}", input.len());

                        if input[index] == 1 {
                            neighbors += 1;
                        }
                    }
                }

                if self.b_list.contains(&neighbors) {
                    output[current_cell] = 1;
                } else if self.s_list.contains(&neighbors) {
                    output[current_cell] = input[current_cell];
                } else {
                    output[current_cell] = 0;
                }
            }
        }

        output
    }
}
