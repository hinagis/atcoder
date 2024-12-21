fn main() {
    proconio::input! {
        n: usize,
        h: [u32; n]
    }
    let mut c = 1;
    for b in 1..n {
        for i in 0..=n - b {
            let mut t = 1;
            let mut p = h[i];
            for j in (i + b..n).step_by(b) {
                if h[j] == p {
                    t += 1;
                    c = c.max(t);
                } else {
                    p = h[j];
                    t = 1;
                }
            }
        }
    }
    println!("{}", c);
}
