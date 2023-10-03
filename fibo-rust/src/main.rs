use malachite::num::basic::traits::{Zero, One};
use malachite::Integer;

fn fibo(i: &Integer) -> Integer {
    if i == &Integer::ZERO {
        Integer::ZERO
    } else if i == &Integer::ONE {
        Integer::ONE
    } else {
        fibo(&(i - &Integer::ONE)) + fibo(&(i - Integer::from(2usize)))
    }
}

fn main() {
    println!("{}", fibo(&Integer::from(42usize)));
}
