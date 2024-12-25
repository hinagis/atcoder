fn main() {
    proconio::input! {s: String}
    println!("{}", s.bytes().fold(0, |s, b| s + (b - b'0') as u32) * 111);
}
