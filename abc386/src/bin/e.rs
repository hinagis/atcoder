use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [u64; n]
    }
    let mut m = 0;
    for c in (0..n).combinations(k) {
        let mut t = 0;
        for i in c {
            t ^= a[i];
        }
        m = m.max(t);
    }
    println!("{}", m);
}
