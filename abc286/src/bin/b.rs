use regex::Regex;

fn main() {
    proconio::input! {
        _: usize,
        s: String
    }
    let re = Regex::new(r"na").unwrap();
    println!("{}", re.replace_all(&s, r"nya"));
}
