fn main() {
    proconio::input! {
        _: usize,
        s: String
    }
    println!("{}", if s.ends_with("tea") {"Yes"} else {"No"});
}
