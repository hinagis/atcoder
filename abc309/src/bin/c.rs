fn main() {
    proconio::input! {
        n: usize,
        k: u64,
        ab: [(u64, u64); n]
    }
    let mut l = 1;
    let mut r = 1_000_000_001;
    while l < r {
        let m = (l + r) / 2;
        let mut s = 0;
        for &(a, b) in &ab {
            if m <= a {
                s += b;
            }
        }
        if s > k {
            l = m + 1;
        } else {
            r = m;
        }
    }
    println!("{}", l);
}
