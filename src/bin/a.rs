fn main() {
    proconio::input! {s: [String]}
    println!("{}", s.iter().filter(|&s| s == "Takahashi").count());
}
