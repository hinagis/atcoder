fn main() {
    proconio::input! {x: i64}
    println!("{}", x / 10 + if x > 0 {if x % 10 == 0 {0} else {1}} else {0});
}
