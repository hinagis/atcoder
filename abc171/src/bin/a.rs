fn main() {
    proconio::input! { a: char }
    println!("{}", if a.is_ascii_lowercase() {"a"} else {"A"});
}
