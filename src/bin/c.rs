fn main() {
    proconio::input! {
        n: usize,
        mut t: u64,
        a: [u64; n],
    }
    t -= {let s: u64 = a.iter().sum(); (t / s) * s};
    for i in 0..n {
        if t > a[i] {
            t -= a[i];
        } else {
            println!("{} {}", i + 1, t);
            break;
        }
    }
}
