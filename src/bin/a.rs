fn main() {
    proconio::input! {
        _: usize,
        d: usize,
        s: String
    }
    println!("{}", s.chars().filter(|&c| c == '.').count() + d);
}
