use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        l: usize,
        r: usize,
    }
    let mut s = (1..l).chain((l..=r).rev()).chain(r + 1..=n);
    println!("{}", s.join(" "));
}
