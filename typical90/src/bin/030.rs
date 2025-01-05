fn main() {
    proconio::input! {
        n: usize,
        k: usize
    }
    let mut c = vec![0; n + 1];
    for f in 2..=n {
        if c[f] > 0 {
            continue;
        }
        for v in (f..=n).step_by(f) {
            c[v] += 1;
        }
    }
    println!("{}", c.iter().filter(|&&c| c >= k).count());
}
