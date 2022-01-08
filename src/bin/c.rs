use proconio::{input as I, marker::Usize1 as U1};

fn main() {
    I! {n: usize}
    let mut t = vec![0; n];
    let mut a = vec![vec![]; n];
    for i in 0..n {
        I! {c: u64, k: usize, b: [U1; k]}
        t[i] = c;
        a[i] = b;
    }

    let mut d = 0;
    let mut f = vec![false; n];
    f[n - 1] = true;
    for i in (0..n).rev() {
        if f[i] {
            d += t[i];
            for &j in &a[i] {
                f[j] = true;
            }
        }
    }

    println!("{}", d);
}
