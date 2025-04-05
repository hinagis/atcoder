fn main() {
    proconio::input! {a: i32}
    println!("{}", if 400 % a == 0 {400 / a} else {-1});
}
