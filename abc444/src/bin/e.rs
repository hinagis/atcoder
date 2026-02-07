use proconio::input as I;
// use proconio::{input as I, fastout as F};

// #[F]
fn main() {
    I! {
        n: usize,
        d: i64,
        a: [i64; n]
    }
    let mut r = 0;
    let mut t = std::collections::BTreeMap::new();
    for l in 0..n {

        if let Some((b, c)) = t.range(..=a[r]).next_back() {
            if (a[r] - b).abs() < d {

            }
        }
        let c = t.range(a[r]..).next();
    }
    println!("{}", n);
}
