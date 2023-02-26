fn main() {
    proconio::input! {s: String}
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            println!("{}", i + 1);
            return;
        }
    }
}
