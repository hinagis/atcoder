fn main() {
    proconio::input! {x: u32}
    println!("{}", if x >= 100 && x % 100 == 0 {"Yes"} else {"No"});
}
