fn main() {
    proconio::input! {s: String}
    println!("{}", s.chars().filter(|&c| c != '.').collect::<String>());
}
