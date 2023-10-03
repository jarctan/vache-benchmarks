use malachite::num::basic::traits::{Zero, One};
use malachite::Integer;
use rand::Rng;

fn partition(array: &mut [Integer], start: &Integer, end: &Integer) -> Integer {
    let pivot = array[usize::try_from(end).unwrap()].clone();

    let mut i: Integer = start.clone() - Integer::ONE;
    let mut j: Integer = start.clone();

    while j < *end {
        if array[usize::try_from(&j).unwrap()] <= pivot {
            i += Integer::ONE;
            array.swap(usize::try_from(&i).unwrap(), usize::try_from(&j).unwrap());
        }
        j += Integer::ONE;
    }

    array.swap(
        usize::try_from(&(&i + Integer::ONE)).unwrap(),
        usize::try_from(end).unwrap(),
    );

    i + Integer::ONE
}

fn _quick_sort(array: &mut [Integer], start: &Integer, end: &Integer) {
    if start < end {
        let pivot = partition(array, start, &end);
        _quick_sort(array, start, &(&pivot - Integer::ONE));
        _quick_sort(array, &(&pivot + Integer::ONE), end);
    }
}

fn quick_sort(array: &mut [Integer]) {
    let start = Integer::ZERO;
    let end = Integer::from(array.len()) - Integer::ONE;
    _quick_sort(array, &start, &end);
}

fn main() {
    let n = Integer::from(100000usize);
    let mut array: Vec<Integer> = Vec::new();
    let mut rng = rand::thread_rng();

    let mut j = Integer::ZERO;
    while j < n {
        array.push(Integer::from(rng.gen_range(0usize..100usize)));
        j += Integer::ONE;
    }

    quick_sort(&mut array);

    dbg!(&array[usize::try_from(&Integer::from(99999usize)).unwrap()]);
}
