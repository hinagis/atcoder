fn main() {
    proconio::input! {
        s: [String],
        x: usize,
        y: String
    }
    println!("{}", if y == s[x - 1] {"Yes"} else {"No"});
}
