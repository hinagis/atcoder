fn main() {
    proconio::input! {
        s: u32,
        t: u32
    }
    let mut r = 0;
    for a in 0..=s {
        for b in 0..=s - a {
            for c in 0..=s - a - b {
                if a * b * c <= t {
                    r += 1
                }
            }
        }
    }
    println!("{}", r);
}
