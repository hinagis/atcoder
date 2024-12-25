fn main() {
    proconio::input! {
        n: usize,
        ab: [(u64, u64); n]
    }
    let mut a = 0;
    let mut d = vec![(0, 0); n];
    for i in 0..n {
        a += ab[i].0;
        d[i].0 = ab[i].0;
        d[i].1 = 2 * ab[i].0 + ab[i].1;
    }
    d.sort_by(|a, b| b.1.cmp(&a.1));

    let mut b = 0;
    for i in 0..n {
        a -= d[i].0;
        b += d[i].1 - d[i].0;
        if b > a {
            println!("{}", i + 1);
            return;
        }
    }
}
