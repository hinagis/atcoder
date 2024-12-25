use proconio::{input as I};

fn main() {
    I! {
        n: usize,
        m: usize,
        a: [u32; n]
    }

    let mut s = 0;
    for _ in 0..m {
        I! {b: usize}
        s += a[b - 1];
    }
    println!("{}", s);
}
