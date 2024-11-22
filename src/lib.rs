use ndarray::prelude::*;

#[derive(Debug)]
pub struct Grid {
    grid: Array2<Option<char>>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            grid: Array::from_elem((3, 4), None),
        }
    }
}
