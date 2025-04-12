fn main() {
    proconio::input! {a: u32}
    println!("{}", if a >= 200 && a < 300 {"Success"} else {"Failure"});
}
