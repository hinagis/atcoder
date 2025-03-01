use std::i32;

use proconio::input as I;

fn main() {
    I! {n: i32}
    let mut m = i32::MAX;
    let mut p = std::collections::HashMap::new();
    for i in 0..n {
        I! {a: i32}
        if let Some(j) = p.get_mut(&a) {
            m = m.min(i - *j + 1);
            *j = i;
        } else {
            p.insert(a, i);
        }
    }
    println!("{}", if m == i32::MAX {-1} else {m});
}
