fn main() {
    proconio::input! {
        n: usize,
        s: u32,
        k: u32,
        c: [(u32, u32); n]
    }
    let c = c.iter().fold(0, |s, (p, q)| s + p * q);
    println!("{}", c + if c >= s {0} else {k});
}
