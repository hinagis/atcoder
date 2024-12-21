use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
        mut r: [u64; n],
    }
    r.sort();
    let mut c = vec![0; n + 1];
    for i in 0..n {
        c[i + 1] = c[i] + r[i];
    }
    for _ in 0..q {
        I! {x: u64}
        let j = match c.binary_search(&x) {
            Ok(j) => j,
            Err(j) => if j > 0 {j - 1} else {0}
        };

        println!("{}", j);
    }
}
