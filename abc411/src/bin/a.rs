fn main() {
    proconio::input! {
        s: String,
        l: usize
    }
    println!("{}", if s.len() >= l {"Yes"} else {"No"});
}
