fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let mut c = vec![0; n];
    for &(a, b) in ab.iter() {
        if a + b - 3 < n {
            c[a + b - 3] += 1;
        } else {
            c[a + b - n - 3] += 1;
        }
    }
    let mut t = (m - 1) * m / 2;
    for c in c {
        if c > 1 {
            t -= (c - 1) * c / 2;
        }
    }
    println!("{}", t);
}
