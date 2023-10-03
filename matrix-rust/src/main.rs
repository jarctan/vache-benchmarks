use malachite::Integer;
use malachite::num::basic::traits::{Zero, One};

fn main() {
    let n = Integer::from(8000usize);
    // Generate the matrix
    let mut model = vec![];
    let mut j = Integer::ZERO;
    while j < n {
        model.push(j.clone());
        j = j.clone() + Integer::ONE;
    }

    let mut a = vec![];
    let mut i = Integer::ZERO;
    while i < n {
        a.push(model.clone());
        i += Integer::ONE;
    }
    println!("{}", a[3][0]);

    // Transpose the matrix
    let mut i = Integer::ZERO;
    while i < n {
        let mut j = Integer::ZERO;
        while j < i {
            let p1: *mut _ = &mut a[usize::try_from(&j).unwrap()][usize::try_from(&i).unwrap()];
            let p2: *mut _ = &mut a[usize::try_from(&i).unwrap()][usize::try_from(&j).unwrap()];
            unsafe {
                std::ptr::swap(p1, p2);
            }
            j = j + Integer::ONE;
        }
        i = i + Integer::ONE;
    }

    println!("{}", a[3][0]);
}
