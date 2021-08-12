use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let mut d = vec![0; n - 1];
    for i in 0..n - 1{
        d[i] = a[i + 1] - a[i]
    }
    let mut s = d.iter().fold(0, |s, d| s + d.abs());
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            v: i64,
        }
        if l > 1 {
            s -= d[l - 2].abs();
            d[l - 2] += v;
            s += d[l - 2].abs();
        }
        if r < n {
            s -= d[r - 1].abs();
            d[r - 1] -= v;
            s += d[r - 1].abs();
        }
        println!("{}", s);
    }
}
