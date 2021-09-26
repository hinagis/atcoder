fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n],
        x: u64
    }
    let b = a.iter().fold(0, |s, e| s + e);
    let c = x / b;
    let r = x % b;
    let mut d = 0;
    let mut i = 0;
    while d <= r {
        d += a[i];
        i += 1;
    }
    println!("{}", c * n as u64 + i as u64);
}
