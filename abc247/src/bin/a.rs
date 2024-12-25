fn main() {
    proconio::input! {s: String}
    println!("0{}", s.get(0..3).unwrap());
}
