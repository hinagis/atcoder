fn main() {
    proconio::input! {
        n: usize,
        mut w: u64,
        mut ab: [(u64, u64); n],
    }
    let mut v = 0;
    ab.sort_by(|a, b| b.0.cmp(&a.0));
    for i in 0..n {
        let (a, b) = ab[i];
        if w >= b {
            v += b * a;
            w -= b;
        } else {
            v += w * a;
            w = 0;
        }
    }

    println!("{}", v);
}
