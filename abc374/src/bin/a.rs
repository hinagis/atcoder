fn main() {
    proconio::input! {s: String}
    println!("{}", if regex::Regex::new("san$").unwrap().is_match(&s) {"Yes"} else {"No"});
}
