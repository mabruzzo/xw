use ndarray::prelude::*;

fn main() {
    let arr: Array2<char> = Array::from_elem((3, 4), ' ');

    println!("Hello, {arr:?}");
}

#[cfg(test)]
mod tests {

    #[test]
    fn dummy_unit_test() {
        assert_eq!(1, 1);
    }
}
