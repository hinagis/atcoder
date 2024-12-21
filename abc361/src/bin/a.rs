use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        x: u32,
        mut a: [u32; n]
    }
    a.insert(k, x);
    println!("{}", a.iter().join(" "));
}
