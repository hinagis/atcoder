fn main() {
    proconio::input! {
        n: usize,
        mut k: u64,
        x: u64,
        mut a: [u64; n],
    }
    for i in 0..n {
        let b = a[i] / x;
        if k >= b {
            a[i] -= b * x;
            k -= b;
        } else {
            a[i] -= k * x;
            k = 0;
        }
    }
    a.sort();
    println!("{}", if k >= n as u64 {0} else {a.drain(0..n - k as usize).sum()});
}
