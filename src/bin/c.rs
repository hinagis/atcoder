use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
    }
    let b = k / n as u64;
    let k = (k % n as u64) as usize;
    let mut a = Vec::with_capacity(n);
    for i in 0..n {
        input! {ai: u32};
        a.push((ai, i, 0));
    }

    a.sort_by(|a, b| a.0.cmp(&b.0));
    for i in 0..k {
        a[i].2 = 1;
    }

    a.sort_by(|a, b| a.1.cmp(&b.1));
    for i in 0..n {
        println!("{}", b + a[i].2);
    }
}
