use ndarray::prelude::*;

use std::fmt;
use std::ops::Range;

type Square = Option<char>;

#[derive(Debug)]
struct SlotCoords {
    r: Range<usize>, // starting and stopping coordinate along slice axis
    i: usize,        // row / col the slot is in
}

#[derive(Debug)]
pub struct Puzzle {
    grid: Array2<Square>,
    downs: Vec<SlotCoords>,
    //TODO acrosses
}

impl Puzzle {
    //TODO delete?
    //pub fn new() -> Grid {
    //    Grid {
    //        grid: Array::from_elem((3, 4), None),
    //    }
    //}

    fn identify_downs(grid: &Array2<Square>) -> Vec<SlotCoords> {
        let mut downs = vec![];
        // first iterate over top row
        for i in 0..grid.shape()[1] {
            let mut stop = 0usize;
            while stop < grid.shape()[0] {
                let mut start = stop;
                while start < grid.shape()[0] && grid[[start, i]] == None {
                    start += 1;
                }
                stop = start;
                while stop < grid.shape()[0] && grid[[stop, i]] != None {
                    stop += 1;
                }
                if start != stop {
                    downs.push(SlotCoords { r: start..stop, i });
                }
                stop += 1;
            }
        }
        downs
    }

    pub fn from_str(s: &str) -> Puzzle {
        let v: Vec<&str> = s.split('\n').collect();
        let ncols = v[0].len();
        let nrows = v.len();
        let mut grid = Array::from_elem((nrows, ncols), None);

        for i in 0..nrows {
            for (j, chr) in v[i].char_indices() {
                grid[[i, j]] = match chr {
                    '.' => None,
                    other => Some(other),
                }
            }
        }

        Puzzle {
            downs: Puzzle::identify_downs(&grid),
            grid,
        }
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Grid{{\n")?;
        for row in self.grid.rows() {
            write!(f, "  ")?;
            for chr in row.iter().map(|e| e.unwrap_or('.')) {
                write!(f, "{chr}")?;
            }
            write!(f, "\n")?;
        }

        write!(f, "DOWNS:\n")?;
        for coords in self.downs.iter() {
            let slot = self.grid.slice(s![coords.r.clone(), coords.i]);
            for chr in slot.iter().map(|e| e.unwrap_or('.')) {
                write!(f, "{chr}")?;
            }
            write!(f, "\n")?;
        }

        write!(f, "}}")?;

        Ok(())
    }
}
