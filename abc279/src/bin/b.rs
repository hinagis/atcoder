fn main() {
    proconio::input! {
        s: String,
        t: String
    }
    println!("{}", if s.contains(&t) {"Yes"} else {"No"});
}
