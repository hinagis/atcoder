fn main() {
    proconio::input! {
        _: usize,
        a: char,
        b: char,
        s: String
    }
    println!("{}", s.chars().map(|c| if c == a {c} else {b}).collect::<String>());
}
