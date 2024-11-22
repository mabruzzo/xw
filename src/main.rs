use xw::*;

fn main() {
    let crossword_str = "\
.ABC.
DE FG
TROUT
.MNO.\
";

    let puzzle = Grid::from_str(crossword_str);

    println!("Hello, {puzzle:?}");
}

#[cfg(test)]
mod tests {

    #[test]
    fn dummy_unit_test() {
        assert_eq!(1, 1);
    }
}
