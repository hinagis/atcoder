//use proconio::{input, fastout};
use proconio::input;
use regex::Regex;

//#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }
    let r = Regex::new(&"^[a-zA-Z0-9]+$").unwrap();
    if  r.is_match(&s)
     && r.is_match(&t)
     && s.len() + 1 == t.len()
     && t.starts_with(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
