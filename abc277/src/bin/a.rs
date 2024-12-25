use proconio::{input as I};

fn main() {
    I! {
        n: usize,
        x: u32
    }
    for i in 1..=n {
        I! {p: u32}
        if p == x {
            println!("{}", i);
            return;
        }
    }
}
