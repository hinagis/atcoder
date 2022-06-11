use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q],
    }

    a.sort();
    let mut c = vec![0; n + 1];
    for i in 1..=n {
        c[i] = c[i - 1] + a[i - 1];
    }

    for &x in &x {
        let i = a.binary_search(&x).unwrap_or_else(|i| i);
        println!("{}", i * x - c[i] + c[n] - c[i] - (n - i) * x);
    }
}
