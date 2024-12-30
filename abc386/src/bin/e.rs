use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [u64; n]
    }
    let mut m = 0;
    let (t, k) = if k <= n - k {(0, k)} else {(a.iter().fold(0, |t, a| t ^ a), n - k)};
    for c in (0..n).combinations(k) {
        let mut t = t;
        for i in c {
            t ^= a[i];
        }
        m = m.max(t);
    }
    println!("{}", m);
}
