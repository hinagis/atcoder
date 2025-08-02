use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {t: usize}
    for _ in 0..t {
        I! {
            n: usize,
            m: u64,
            mut a: [u64; n],
            mut b: [u64; n]
        }
        a.sort_by(|a, b| b.cmp(a));
        b.sort();
        let mut c = n as u64;
        let mut i = 0;
        for k in 0..n {
            while i < n && a[k] + b[i] < m {
                i += 1;
            }
            if i >= n {
                c = k as u64;
                break;
            }
            i += 1;
        }
        println!("{}", a.iter().sum::<u64>() + b.iter().sum::<u64>() - c * m);
    }
}
