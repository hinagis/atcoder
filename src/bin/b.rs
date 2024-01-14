fn main() {
    proconio::input! {n: u32}
    let mut c = 0;
    while n & (1 << c) == 0 {
        c += 1;
    }
    println!("{}", c);
}
