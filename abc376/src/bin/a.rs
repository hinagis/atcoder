fn main() {
    proconio::input! {
        n: usize,
        c: u32,
        t: [u32; n]
    }
    let mut r = 1;
    let mut p = t[0];
    for i in 1..n {
        if t[i] - p >= c {
            r += 1;
            p = t[i];
        }
    }
    println!("{}", r);
}
