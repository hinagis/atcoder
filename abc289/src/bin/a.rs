fn main() {
    proconio::input! {s: String}
    println!("{}", s.chars().map(|c| if c == '0' {'1'} else {'0'}).collect::<String>());
}
