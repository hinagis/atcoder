use proconio::{input as I, fastout as F};
use superslice::Ext;

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
        mut a: [usize; n]
    }
    a.sort();
    let mut c = vec![0; n + 1];
    for i in 0..n {
        c[i + 1] = c[i] + a[i];
    }
    for _ in 0..q {
        I! {b: usize}
        let i = a.lower_bound(&b);
        if i >= n {
            println!("-1");
        } else {
            println!("{}", c[i] + (b - 1) * (n - i) + 1);
        }
    }
}
