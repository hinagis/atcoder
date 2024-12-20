use proconio::input as I;

fn main() {
    I! {
        n: usize,
        mut r: i32
    }
    for _ in 0..n {
        I! {
            d: u8,
            a: i32
        }
        let s = if d == 1 {(1600, 2799)} else {(1200, 2399)};
        if r >= s.0 && r <= s.1 {
            r += a
        }
    }
    println!("{}", r);
}
