fn main() {
    proconio::input! {
        a: u32,
        b: u32
    }
    let (a, b) = if a > b {(b, a)} else {(a, b)};
    println!("{}", if a == b {1} else if (b - a) % 2 == 0 {3} else {2});
}
