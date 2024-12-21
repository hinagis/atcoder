fn main() {
    proconio::input! {s: String}
    println!("{}", regex::Regex::new(r"\|.*\|").unwrap().replace(&s, ""));
}
