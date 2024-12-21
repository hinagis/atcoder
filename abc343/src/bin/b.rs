use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        a: [[usize; n]; n]
    }
    for i in 0..n {
        println!("{}", a[i].iter()
            .enumerate()
            .filter(|(_, &e)| e == 1)
            .map(|(i, _)| (i + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
        );
    }
}
