fn main() {
    proconio::input! {n: u32}
    println!("{}", (1..=n).into_iter().map(|i| (if i % 3 == 0 {'x'} else {'o'})).collect::<String>());
}
