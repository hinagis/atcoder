use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        m: usize,
        ab: [(U, u64); n]
    }
    let mut w = vec![(0, 0); m];
    for (a, b) in ab {
        w[a].0 += b;
        w[a].1 += 1;
    }
    for i in 0..m {
        println!("{}", w[i].0 as f64 / w[i].1 as f64);
    }
}
