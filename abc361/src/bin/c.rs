fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        mut a: [u64; n]
    }
    a.sort();
    let mut m = u64::MAX;
    for i in 0..=k {
        m = m.min(a[n - k - 1 + i] - a[i]);
    }
    println!("{}", m);
}
