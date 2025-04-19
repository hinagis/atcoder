fn main() {
    proconio::input! {s: String}
    println!("{}", s.chars().filter(|c| c.is_uppercase()).collect::<String>());
}
