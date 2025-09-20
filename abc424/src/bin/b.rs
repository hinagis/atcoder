use itertools::Itertools;
use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        m: usize,
        k: u8
    }
    let mut c = vec![0; n];
    let mut g = vec![];
    for _ in 0..k {
        I! {
            a: U,
            _: u8
        }
        c[a] += 1;
        if c[a] >= m {
            g.push(a + 1);
        }
    }
    if g.is_empty() {
        return;
    }
    println!("{}", g.iter().join(" "));
}
