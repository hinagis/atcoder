use regex::Regex;

fn main() {
    proconio::input! {s: String}
    println!("{}", if Regex::new(r"^<=+>$").unwrap().is_match(&s) {"Yes"} else {"No"});
}
