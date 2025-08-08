//! Implements a crossword puzzle container

use ndarray::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

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

    pub fn is_empty(&self) -> bool {
        self.view.is_empty()
    }
}

impl Index<usize> for Slot<'_> {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(ref letter) = self.view[index] {
            letter
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

/// Puzzle grid state.
/// accessing slots is done by index, with no direct grid access, and no distinguishing between
/// across and down slots.
#[derive(Clone, Debug)]
pub struct Puzzle {
    grid: Array2<Square>,
    downs: Vec<SlotCoords>,
    acrosses: Vec<SlotCoords>,
}

impl Puzzle {
    fn identify_slots(grid: &Array2<Square>) -> (Vec<SlotCoords>, Vec<SlotCoords>) {
        let mut downs = vec![];
        let mut acrosses = vec![];

        // first downs, then acrosses
        for (rows_or_cols, slots) in [(grid.columns(), &mut downs), (grid.rows(), &mut acrosses)] {
            // first iterate over top row
            for (k, rowcol) in rows_or_cols.into_iter().enumerate() {
                let mut stop = 0usize; // cursor for slot end position

                // until we hit the end of the row/column
                while stop < rowcol.len() {
                    let mut start = stop; // cursor for slot start position

                    // find the first fillable square
                    while start < rowcol.len() && rowcol[start].is_none() {
                        start += 1;
                    }

                    // find the last following fillable square
                    stop = start;
                    while stop < rowcol.len() && !rowcol[stop].is_none() {
                        stop += 1;
                    }

                    if start != stop {
                        slots.push(SlotCoords { r: start..stop, k });
                    }
                    stop += 1;
                }
            }
        }
        (acrosses, downs)
    }

    /// Construct a Puzzle from a string view
    ///
    /// #Note About Grapheme Clusters
    /// [grapheme clusters](https://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries)
    /// are a subtle aspect of unicode.
    /// - in short, a "user-perceived character" may correspond to a cluster
    ///   of one or more unicode characters.
    /// - As I understand it, you can think of most of these characters as
    ///   "modifiers" (I believe a g with grave-accent is a "g" followed by a
    ///   grave-modifier character). BE AWARE: This mental model may not apply
    ///   for some characters used to represent non-latin-alphabet languages.
    /// - In any case, a grapheme cluster is an approximation for these
    ///   clusters of letters
    ///
    /// In the future, a crossword puzzle should fully support arbitrary
    /// grapheme clusters. For now, this constructor, will parse the cluster,
    /// and report an error.
    pub fn parse(s: &str) -> Result<Puzzle, &'static str> {
        let v: Vec<&str> = s.split('\n').collect();
        // true to use extended, as opposed to legacy grapheme clusters
        let ncols = v[0].graphemes(true).count();
        let nrows = v.len();
        let mut grid = Array::from_elem((nrows, ncols), None);

        for i in 0..nrows {
            let mut j = 0;
            for grapheme in UnicodeSegmentation::graphemes(v[i], true) {
                if j == ncols {
                    // with custom error types, we coud be more descriptive
                    return Err("a row has too many characters");
                }

                let mut inner_it = grapheme.chars();
                // based on my understanding of invariants, the following never panics!
                let chr = inner_it.next().unwrap();
                if let Some(_dummy) = inner_it.next() {
                    return Err("crossword puzzle can't contain a grapheme cluster composed of more than 1 unicode character");
                } else {
                    grid[[i, j]] = match chr {
                        '.' => None,
                        other => Some(other),
                    }
                }
                j += 1;
            }
            if j != ncols {
                return Err("a row has too few characters");
            }
        }

        let (acrosses, downs) = Puzzle::identify_slots(&grid);
        Ok(Puzzle {
            grid,
            acrosses,
            downs,
        })
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
    writeln!(f)
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid{{")?;
        for row in self.grid.rows() {
            fmt_squares(f, row.iter(), Some("  "))?;
        }
        writeln!(f)?;

        writeln!(f, "ACROSSES:")?;
        for coords in self.acrosses.iter() {
            let slot = self.grid.slice(s![coords.k, coords.r.clone()]);
            fmt_squares(f, slot.iter(), Some(" ->"))?;
        }
        writeln!(f)?;

        writeln!(f, "DOWNS:")?;
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
    fn puzzle_creation_errors() {
        let too_few_chars = "\
.ABC.
DE.F\
";
        assert!(
            Puzzle::parse(too_few_chars).is_err(),
            "too few characters in the second row"
        );

        let too_many_chars = "\
.ABC.
DE.FGH\
";
        assert!(
            Puzzle::parse(too_many_chars).is_err(),
            "too few characters in the second row"
        );

        let multi_character_grapheme = "\
.a̐BC.
DE.FG\
";
        assert!(
            Puzzle::parse(multi_character_grapheme).is_err(),
            "can't currently handle a multi-character grapheme cluster"
        );
    }

    #[test]
    fn basic() {
        let crossword_str = "\
.ABC.
DE.FG
TROUT
.MNO.\
";

        let puzzle = Puzzle::parse(crossword_str).unwrap();
        println!("{}", &puzzle);

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

        for (i, val) in ref_vals.iter().enumerate() {
            assert_eq!(String::from(puzzle.access(i)), *val);
        }

        // we are going to modify modify_index so that it now holds "XY"
        let modify_index = puzzle.nslots() - 1;
        let new_val = "XY";
        let mut modified_ref_vals = ref_vals.clone();
        modified_ref_vals[2] = String::from("FX");
        modified_ref_vals[3] = String::from("TROUY");
        modified_ref_vals[modify_index] = new_val.to_string();

        let modified = puzzle.with_filled_slot(modify_index, new_val);

        // ensure the original wasn't modified
        for (i, val) in ref_vals.iter().enumerate() {
            assert_eq!(String::from(puzzle.access(i)), *val);
        }

        // ensure the modified puzzle has the updated values
        for (i, val) in modified_ref_vals.iter().enumerate() {
            assert_eq!(String::from(modified.access(i)), *val);
        }
    }
}
