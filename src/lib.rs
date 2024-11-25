//! Implements a crossword puzzle container

use ndarray::prelude::*;

use std::convert::From;
use std::fmt;
use std::ops::{Index, Range};
use std::string::String;

type Square = Option<char>;

/// A read-only view of a crossword slot
///
/// This needs to be a newly defined type so that we can implement the From trait for
/// ergonomic string conversions
pub struct Slot<'a> {
    view: ArrayView1<'a, Square>,
}

impl Slot<'_> {
    pub fn len(&self) -> usize {
        self.view.len()
    }
}

impl Index<usize> for Slot<'_> {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(ref index) = self.view[index] {
            index
        } else {
            panic!("A slot instance should not hold an empty Square");
        }
    }
}

impl From<Slot<'_>> for String {
    fn from(item: Slot) -> String {
        String::from_iter(item.view.iter().map(|elem| -> char {
            if let Some(chr) = elem {
                *chr
            } else {
                panic!("A slot instance should not hold an empty Square");
            }
        }))
    }
}

/// used internally within a struct
#[derive(Clone, Debug)]
struct SlotCoords {
    r: Range<usize>, // starting and stopping coordinate along slice axis
    k: usize,        // row / col the slot is in
}

/// Abstracts the puzzle
#[derive(Clone, Debug)]
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

    /// Get the number of across-slots
    pub fn nacross(&self) -> usize {
        self.acrosses.len()
    }

    /// Get number of down-slots
    pub fn ndown(&self) -> usize {
        self.downs.len()
    }

    pub fn nslots(&self) -> usize {
        self.nacross() + self.ndown()
    }

    /// returns a read-only view of a crossword slot
    pub fn access(&self, i: usize) -> Slot {
        if i < self.nacross() {
            let coords: &SlotCoords = &self.acrosses[i];
            Slot {
                view: self.grid.slice(s![coords.k, coords.r.clone()]),
            }
        } else if i < self.nslots() {
            let coords: &SlotCoords = &self.downs[i - self.nacross()];
            Slot {
                view: self.grid.slice(s![coords.r.clone(), coords.k]),
            }
        } else {
            panic!("The index is too large!");
        }
    }

    /// creates a new copy with a filled in puzzle
    ///
    /// This interface provides the desired CoW semantics (even if we don't
    /// currently take advantage of them)
    pub fn with_filled_slot(&self, i: usize, value: &str) -> Self {
        let mut copy = self.clone();
        // this could be more efficient
        if value.chars().count() != copy.access(i).len() {
            panic!("value doesn't have the correct length");
        }

        let mut view = if i < self.nacross() {
            let coords: &SlotCoords = &copy.acrosses[i];
            copy.grid.slice_mut(s![coords.k, coords.r.clone()])
        } else {
            let coords: &SlotCoords = &self.downs[i - self.nacross()];
            copy.grid.slice_mut(s![coords.r.clone(), coords.k])
        };

        for (j, chr) in value.char_indices() {
            view[j] = Some(chr);
        }
        copy
    }
}

fn fmt_squares<'a, I>(f: &mut fmt::Formatter<'_>, squares: I, indent: Option<&str>) -> fmt::Result
where
    I: Iterator<Item = &'a Square>,
{
    if let Some(indent_str) = indent {
        write!(f, "{indent_str}")?;
    }
    for square in squares {
        write!(f, "{}", square.unwrap_or('.'))?;
    }
    write!(f, "\n")
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Grid{{\n")?;
        for row in self.grid.rows() {
            fmt_squares(f, row.iter(), Some("  "))?;
        }
        write!(f, "\n")?;

        write!(f, "ACROSSES:\n")?;
        for coords in self.acrosses.iter() {
            let slot = self.grid.slice(s![coords.k, coords.r.clone()]);
            fmt_squares(f, slot.iter(), Some(" ->"))?;
        }
        write!(f, "\n")?;

        write!(f, "DOWNS:\n")?;
        for coords in self.downs.iter() {
            let slot = self.grid.slice(s![coords.r.clone(), coords.k]);
            fmt_squares(f, slot.iter(), Some(" ->"))?;
        }

        write!(f, "}}")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;

    #[test]
    fn basic() {
        let crossword_str = "\
.ABC.
DE.FG
TROUT
.MNO.\
";

        let puzzle = Puzzle::from_str(crossword_str);

        let across_vals = ["ABC", "DE", "FG", "TROUT", "MNO"];
        let down_vals = ["DT", "AERM", "B", "ON", "CFUO", "GT"];
        assert_eq!(puzzle.nacross(), across_vals.len());
        assert_eq!(puzzle.ndown(), down_vals.len());

        let ref_vals = Vec::from_iter(
            across_vals
                .iter()
                .chain(down_vals.iter())
                .map(|s| s.to_string()),
        );
        assert_eq!(puzzle.nslots(), ref_vals.len());

        for i in 0..puzzle.nslots() {
            assert_eq!(String::from(puzzle.access(i)), ref_vals[i]);
        }

        // we are going to modify modify_index so that it now holds "XY"
        let modify_index = puzzle.nslots() - 1;
        let new_val = "XY";
        let mut modified_ref_vals = ref_vals.clone();
        modified_ref_vals[2] = String::from("FX");
        modified_ref_vals[3] = String::from("TROUY");
        modified_ref_vals[modify_index] = new_val.to_string();

        let modified = puzzle.with_filled_slot(modify_index, &new_val);

        // ensure the original wasn't modified
        for i in 0..puzzle.nslots() {
            assert_eq!(String::from(puzzle.access(i)), ref_vals[i]);
        }

        // ensure the modified puzzle has the updated values
        for i in 0..modified.nslots() {
            assert_eq!(String::from(modified.access(i)), modified_ref_vals[i]);
        }
    }
}
