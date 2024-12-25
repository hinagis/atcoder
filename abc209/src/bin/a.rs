fn main() {
    proconio::input! {a: u32, b: u32}
    println!("{}", (b + 1).saturating_sub(a));
}
