fn main() {
    proconio::input! {
        a: u8,
        b: u8,
        c: u8
    }
    println!("{}", if a == b || b == c || c == a  {"Yes"} else {"No"});
}
