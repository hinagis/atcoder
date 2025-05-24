fn main() {
    proconio::input! {
        a: u32,
        b: u32
    }
    let (c, d) = (a / b, a % b);
    println!("{}", c + if d * 2 > b {1} else {0});
}
