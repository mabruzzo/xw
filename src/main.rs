use std::error::Error;
use xw::puzzle::*;

fn main() -> Result<(), Box<dyn Error>> {
    let crossword_str = "\
.ABC..asdf
DE FG.asdf
TROUT.asdf
.MNO..asdf\
";

    let puzzle = Puzzle::parse(crossword_str)?;

    let lex = xw::lexicon::Lexicon::from_file("words.txt")?;

    println!("This is our puzzle: {puzzle}");

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn dummy_unit_test() {
        assert_eq!(1, 1);
    }
}
