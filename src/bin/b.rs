fn main() {
    proconio::input! {x: i64}
    println!("{}", (if x < 0 {x - 9} else {x}) / 10);
}
