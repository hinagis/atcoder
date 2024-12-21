fn main() {
    proconio::input! {
        k: u32,
        g: u32,
        m: u32
    }
    let mut f = 0;
    let mut n = 0;
    for _ in 0..k {
        if f >= g {
            f = 0;
        } else if n == 0 {
            n = m;
        } else {
            let d = g - f;
            if d >= n {
                f += n;
                n = 0;
            } else {
                f = g;
                n -= d;
            }
        }
    }
    println!("{f} {n}");
}
