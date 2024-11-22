use ndarray::prelude::*;

use std::fmt;

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

    pub fn from_str(s: &str) -> Grid {
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
        Grid { grid }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Grid{{\n")?;
        for row in self.grid.rows() {
            write!(f, "  ")?;
            for chr in row.iter().map(|e| e.unwrap_or('■')) {
                write!(f, "{chr}")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
