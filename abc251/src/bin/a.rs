fn main() {
    proconio::input! {s: String}
    println!("{}", s.repeat(6 / s.len()));
}
