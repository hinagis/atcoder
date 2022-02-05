fn main() {
    proconio::input! {n: i64}
    println!("{}", if n == (n as i32) as i64 {"Yes"} else {"No"});
}
