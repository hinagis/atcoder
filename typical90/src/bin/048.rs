fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        ab: [(u64, u64); n]
    }

    let mut d = vec![0; 2 * n];
    for i in 0..n {
        let (a, b) = ab[i];
        d[i * 2] = b;
        d[i * 2 + 1] = a - b;
    }
    d.sort_by(|a, b| b.cmp(a));

    let mut v = 0u64;
    for i in 0..k.min(2 * n) {
        v += d[i];
    }
    println!("{}", v);
}
