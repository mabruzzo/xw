use xw::*;

fn main() {
    let crossword_str = "\
.ABC..asdf
DE FG.asdf
TROUT.asdf
.MNO..asdf\
";

    let puzzle = Puzzle::from_str(crossword_str);

    println!("This is our puzzle: {puzzle}");
}

#[cfg(test)]
mod tests {

    #[test]
    fn dummy_unit_test() {
        assert_eq!(1, 1);
    }
}
