fn main() {
    proconio::input! {s: String}
    let r = regex::Regex::new(r"^[A-Z][1-9][0-9]{5}[A-Z]$").unwrap();
    println!("{}", if r.is_match(&s) {"Yes"} else {"No"});
}
