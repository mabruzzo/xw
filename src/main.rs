use xw::*;

fn main() {
    let puzzle = Grid::new();

    println!("Hello, {puzzle:?}");
}

#[cfg(test)]
mod tests {

    #[test]
    fn dummy_unit_test() {
        assert_eq!(1, 1);
    }
}
