fn main() {
    proconio::input! {
        a: usize,
        b: usize,
    }
    let c = 2 * a + 100;
    println!("{}", if c > b {c - b} else {0});
}
