use ndarray::prelude::*;

use std::fmt;
use std::ops::Range;

type Square = Option<char>;

#[derive(Debug)]
struct SlotCoords {
    r: Range<usize>, // starting and stopping coordinate along slice axis
    k: usize,        // row / col the slot is in
}

#[derive(Debug)]
pub struct Puzzle {
    grid: Array2<Square>,
    downs: Vec<SlotCoords>,
    acrosses: Vec<SlotCoords>,
}

impl Puzzle {
    //TODO delete?
    //pub fn new() -> Grid {
    //    Grid {
    //        grid: Array::from_elem((3, 4), None),
    //    }
    //}

    fn identify_slots(grid: &Array2<Square>) -> (Vec<SlotCoords>, Vec<SlotCoords>) {
        let mut downs = vec![];
        let mut acrosses = vec![];

        for (axes, collection) in [(grid.columns(), &mut downs), (grid.rows(), &mut acrosses)] {
            // first iterate over top row
            for (k, axis) in axes.into_iter().enumerate() {
                let mut stop = 0usize;
                let n = axis.len();
                while stop < n {
                    let mut start = stop;
                    while start < n && axis[start] == None {
                        start += 1;
                    }
                    stop = start;
                    while stop < n && axis[stop] != None {
                        stop += 1;
                    }
                    if start != stop {
                        collection.push(SlotCoords { r: start..stop, k });
                    }
                    stop += 1;
                }
            }
        }
        (acrosses, downs)
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

        let (acrosses, downs) = Puzzle::identify_slots(&grid);
        Puzzle {
            grid,
            acrosses,
            downs,
        }
    }
}

//fn fmt_squares<I>(f: &mut fmt::Formatter<'_>, squares: I) -> fmt::Result
//where
//    I: for<'a> Iterator<Item = &'a Square>,
//{
//    for chr in squares.map(|e| e.unwrap_or('.')) {
//
//        write!(f, "{chr}")?;
//    }
//    Ok(())
//}

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
        write!(f, "\n")?;

        write!(f, "ACROSSES:\n")?;
        for coords in self.acrosses.iter() {
            let slot = self.grid.slice(s![coords.k, coords.r.clone()]);
            write!(f, " ->")?;
            for chr in slot.iter().map(|e| e.unwrap_or('.')) {
                write!(f, "{chr}")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")?;

        write!(f, "DOWNS:\n")?;
        for coords in self.downs.iter() {
            let slot = self.grid.slice(s![coords.r.clone(), coords.k]);
            write!(f, " ->")?;
            for chr in slot.iter().map(|e| e.unwrap_or('.')) {
                write!(f, "{chr}")?;
            }
            write!(f, "\n")?;
        }

        write!(f, "}}")?;

        Ok(())
    }
}
