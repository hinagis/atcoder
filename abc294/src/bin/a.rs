fn main() {
    proconio::input! {a: [u32]}
    println!("{}", a.iter().filter(|&&a| a % 2 == 0).map(|a| a.to_string()).collect::<Vec<_>>().join(" "));
}
