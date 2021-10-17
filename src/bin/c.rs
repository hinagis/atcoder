fn main() {
    proconio::input! {
        n: usize,
        ab: [(f64, f64); n]
    }
    let mut c = vec![0f64; n];
    for i in 0..n {
        let (a, b) = ab[i];
        c[i] = a / b;
    }
    let mut l = (0, 0f64);
    let mut r = (n - 1, 0f64);
    while l.0 < r.0 {
        if c[l.0] - l.1 > c[r.0] - r.1 {
            l.1 += c[r.0] - r.1;
            r.0 -= 1;
            r.1 = 0f64;
        } else if c[l.0] - l.1 < c[r.0] - r.1 {
            r.1 += c[l.0] - l.1;
            l.0 += 1;
            l.1 = 0f64;
        } else {
            l.0 += 1;
            r.0 -= 1;
            if l.0 > r.0 {
                let mut s = 0f64;
                for i in 0..l.0 {
                    s += ab[i].0;
                }
                println!("{}", s);
                return;
            }
            l.1 = 0f64;
            r.1 = 0f64;
        }
    }

    let mut s = 0f64;
    for i in 0..l.0 {
        s += ab[i].0;
    }
    s += ab[l.0].1 * (l.1 + (c[l.0] -l.1 - r.1)/ 2f64);
    println!("{}", s);
    dbg!(&l, &r);
}
