fn main() {
    proconio::input! {a: u64, b: u32, c: u64}
    println!("{}", if a < c.pow(b) {"Yes"} else {"No"});
}