fn main() {
    proconio::input! {s: String}
    println!("{}", if s.ends_with("r") {"er"} else {"ist"});
}
