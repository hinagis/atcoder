fn main() {
    proconio::input! {n: String}
    println!("{}", n.chars().collect::<Vec<_>>()[1..].iter().collect::<String>());
}
