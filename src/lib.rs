use ndarray::prelude::*;

#[derive(Debug)]
pub struct Grid {
    grid: Array2<char>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            grid: Array::from_elem((3, 4), ' '),
        }
    }
}
