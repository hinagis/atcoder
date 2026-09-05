 use proconio::{input as I, fastout as F};

 #[F]
fn main() {
    I! {
        n: usize,
        m: usize,
        k: u64,
        a: [u64; n],
    }
    let mut s = 0;
    let mut h = vec![false; n];
    for i in 0..n {
        if i >= m && h[i - m] {
            s -= a[i - m];
        }
        println!("{}", 
            if s + a[i] <= k {
                s += a[i];
                h[i] = true;
                "Yes"
            } else {
                "No"
            });
    }
}
