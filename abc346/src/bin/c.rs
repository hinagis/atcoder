fn main() {
    proconio::input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }
    let e: std::collections::HashSet<u64> = a.iter().filter(|&&a| a <= k).cloned().collect();
    println!("{}", e.iter().fold(k * (k + 1) / 2, |r, e| r - e));
}
