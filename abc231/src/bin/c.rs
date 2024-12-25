use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u32; n]
    }
    a.sort();
    let a: Vec<u32> = a;
    for _ in 0..q {
        proconio::input! {x: u32}
        println!("{}", n - match a.binary_search(&x) {Ok(n) => n, Err(n) => n});
    }
}
