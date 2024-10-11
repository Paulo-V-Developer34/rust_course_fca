use std::ops::{Range, RangeInclusive};

fn main() {
    assert_eq!((1..5), Range { start: 1, end: 5 }); //praticamente ninguém utiliza isso
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); //os dois são praticamente a mesma coisa
    println!("success")
}
