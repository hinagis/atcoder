use regex::Regex;

fn main() {
    proconio::input! {s: String}
    println!("{}", if Regex::new(r"^A*B*C*$").unwrap().is_match(s.as_str()) {"Yes"} else {"No"});
}
