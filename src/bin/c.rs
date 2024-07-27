fn main() {
    proconio::input! {
        n: usize,
        x: u64,
        y: u64,
        mut a: [u64; n],
        mut b: [u64; n],
    }
    a.sort_by(|a, b| b.cmp(a));
    b.sort_by(|a, b| b.cmp(a));
    let mut s = (0, 0);
    for i in 0..n {
        s.0 += a[i];
        s.1 += b[i];
        if s.0 > x || s.1 > y {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", n);
}
